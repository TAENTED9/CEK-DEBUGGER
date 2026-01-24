use super::{
    Context, Machine, MachineState, 
    value::Value,
    discharge::value_as_term,
    Error,
};
use crate::ast::{NamedDeBruijn, Term};
use crate::machine::cost_model::{ExBudget, StepKind};
use serde::Serialize;
use std::rc::Rc;

// In crates/uplc/src/machine/debug.rs
#[derive(Clone, Debug, Serialize)]
pub struct StepSnapshot {
    pub step: usize,
    pub state_type: String,
    pub term: String,
    pub environment: Vec<String>,
    pub context_depth: usize,
    pub cpu: i64,
    pub mem: i64,
    
    pub validator_phase: ValidatorPhase,  // Datum check, redeemer check, etc.
    pub current_check: Option<String>,     // Human-readable current validation
    pub script_context: Option<ScriptContextSnapshot>,  // Tx inputs/outputs
}

#[derive(Clone, Debug, Serialize)]
pub enum ValidatorPhase {
    DatumValidation,
    RedeemerValidation,
    SignerCheck,
    ValueCheck,
    TimeRangeCheck,
    Custom(String),
}

#[derive(Clone, Debug, Serialize)]
pub struct ScriptContextSnapshot {
    pub inputs: Vec<TxInputSummary>,
    pub outputs: Vec<TxOutputSummary>,
    pub signatories: Vec<String>,
    pub time_range: Option<(i64, i64)>,
    pub redeemer: String,
}

impl Machine {
    /// Run the machine with step-by-step snapshots for debugging
    pub fn run_debug(&mut self, term: Term<NamedDeBruijn>) -> Result<(Term<NamedDeBruijn>, Vec<StepSnapshot>), Error> {
        use MachineState::*;

        let mut snapshots = Vec::new();
        let mut step_count = 0;

        let startup_budget = self.costs.machine_costs.get(StepKind::StartUp);
        self.spend_budget(startup_budget)?;

        let mut state = Compute(Context::NoFrame, Rc::new(vec![]), term);

        loop {
            // Capture snapshot BEFORE processing this state
            snapshots.push(capture_snapshot(step_count, &state, &self.ex_budget));
            step_count += 1;

            state = match state {
                Compute(context, env, t) => self.compute(context, env, t)?,
                Return(context, value) => self.return_compute(context, value)?,
                Done(t) => {
                    // Final snapshot
                    snapshots.push(StepSnapshot {
                        step: step_count,
                        state_type: "Done".to_string(),
                        term: format!("{}", t),
                        environment: vec![],
                        context_depth: 0,
                        cpu: self.ex_budget.cpu,
                        mem: self.ex_budget.mem,
                    });
                    return Ok((t, snapshots));
                }
            };
        }
    }
}

fn capture_snapshot(step: usize, state: &MachineState, budget: &ExBudget) -> StepSnapshot {
    match state {
        MachineState::Compute(context, env, term) => {
            StepSnapshot {
                step,
                state_type: "Compute".to_string(),
                term: format!("{}", term),
                environment: env.iter()
                    .enumerate()
                    .map(|(i, v)| format!("[{}] {}", i, pretty_value(v)))
                    .collect(),
                context_depth: count_context_depth(context),
                cpu: budget.cpu,
                mem: budget.mem,
            }
        }
        MachineState::Return(context, value) => {
            StepSnapshot {
                step,
                state_type: "Return".to_string(),
                term: pretty_value(value),
                environment: vec![],
                context_depth: count_context_depth(context),
                cpu: budget.cpu,
                mem: budget.mem,
            }
        }
        MachineState::Done(term) => {
            StepSnapshot {
                step,
                state_type: "Done".to_string(),
                term: format!("{}", term),
                environment: vec![],
                context_depth: 0,
                cpu: budget.cpu,
                mem: budget.mem,
            }
        }
    }
}

fn pretty_value(value: &Value) -> String {
    // Convert value back to term for display
    let term = value_as_term(value.clone());
    format!("{}", term)
}

fn count_context_depth(context: &Context) -> usize {
    match context {
        Context::NoFrame => 0,
        Context::FrameAwaitArg(_, rest) => 1 + count_context_depth(rest),
        Context::FrameAwaitFunTerm(_, _, rest) => 1 + count_context_depth(rest),
        Context::FrameAwaitFunValue(_, rest) => 1 + count_context_depth(rest),
        Context::FrameForce(rest) => 1 + count_context_depth(rest),
        Context::FrameConstr(_, _, _, _, rest) => 1 + count_context_depth(rest),
        Context::FrameCases(_, _, rest) => 1 + count_context_depth(rest),
    }
}