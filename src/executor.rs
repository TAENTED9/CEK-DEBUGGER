use anyhow::{Result, anyhow};
use uplc::ast::Program;
use uplc::machine::Machine;
use uplc::machine::cost_model::{CostModel, ExBudget};
use uplc::ast::NamedDeBruijn;
use uplc::machine::Trace;

/// Execute the program using the public Machine API and return the machine traces
/// and the final/executed budget.  The old detailed MachineState stepping is private
/// in newer uplc versions so we collect the public traces instead.
///
/// Note: this returns the vector of Trace (human readable labels/logs) and the
/// final ExBudget. If you need full CEK internals (env/context/value) you'll need
/// to implement a CEK interpreter or fork/expose the uplc internals.
pub fn execute_program(program: Program<NamedDeBruijn>) -> Result<(Vec<Trace>, ExBudget)> {
    // Choose debug machine so we get spend counters & better traces in some builds.
    // For the version parameter we use the program version encoded in the Program struct.
    // uplc stores version as a triple (maj, min, patch) in Program::version.
    // The machine expects a Cardano Language type (pallas_primitives::conway::Language).
    // The uplc::machine::Machine stores version: Language internally and Program::version is a tuple.
    // As a pragmatic default we construct the machine with the Babbage/Alonzo language
    // derived from the version major number when possible; if you want exact mapping
    // implement a conversion helper.
    use pallas_primitives::conway::Language;

    let prog_version = program.version;
    // Very small heuristic: program.version.0 >= 6 => choose Language::Babbage, else Alonzo.
    // (Adjust mapping to your target Cardano era as needed.)
    let language = if prog_version.0 >= 6 {
        Language::Babbage
    } else {
        Language::Alonzo
    };

    let mut machine = Machine::new_debug(
        language,
        CostModel::default(),
        ExBudget::default(),
        1u32,
    );

    // run returns the final term or an error.
    let _final_term = machine
        .run(program.term)
        .map_err(|e| anyhow!("machine run error: {}", e))?;

    // machine.traces is public (Vec<Trace>), ex_budget is updated on the machine.
    let traces = machine.traces.clone();
    let budget = machine.ex_budget.clone();

    Ok((traces, budget))
}
