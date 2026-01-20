use anyhow::{Result, anyhow};
use uplc::ast::Program;
use uplc::machine::Machine;
use uplc::machine::debug::StepSnapshot;
use uplc::machine::cost_model::{CostModel, ExBudget};
use uplc::ast::NamedDeBruijn;
use pallas_primitives::conway::Language;

pub fn execute_program(program: Program<NamedDeBruijn>) -> Result<Vec<StepSnapshot>> {
    let mut machine = Machine::new(
        Language::PlutusV2,  // or PlutusV1, PlutusV3 depending on your needs
        CostModel::default(),
        ExBudget::default(),
        1u32,
    );

    let (_final_term, snapshots) = machine
        .run_debug(program.term)
        .map_err(|e| anyhow!("Machine execution error: {}", e))?;

    Ok(snapshots)
}