use anyhow::Result;
use uplc::ast::Program;
use uplc::machine::Machine;
use uplc::machine::debug::StepSnapshot;
use uplc::machine::cost_model::{CostModel, ExBudget};
use uplc::ast::NamedDeBruijn;


pub fn execute_program(program: Program<NamedDeBruijn>) -> Result<Vec<StepSnapshot>> {
    let language = pallas_primitives::conway::Language::PlutusV2;
    let mut machine = Machine::new(
        language,
        CostModel::default(),
        ExBudget::default(),
        1u32,
    );

    let (_final_term, snapshots) = machine
        .run_debug(program.term)
        .map_err(|e| anyhow::anyhow!("Failed to execute UPLC program: {}", e))?;

    Ok(snapshots)
}