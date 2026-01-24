use serde::Serialize;
use uplc::machine::debug::StepSnapshot;
use std::collections::BTreeMap;


#[derive(Clone, Serialize)]
pub struct Frame {
    pub step: usize,
    pub state_type: String,
    
    // REPLACE raw term with semantic description
    pub human_description: String,  // "Checking if value >= 100 ADA"
    pub technical_detail: String,   // Original UPLC term (collapsed by default)
    
    pub environment: Vec<String>,
    pub context_depth: usize,
    pub cpu: i64,
    pub mem: i64,
    pub source_location: Option<String>,
    
    // NEW: Validation context
    pub validation_phase: String,   // "Redeemer validation"
    pub current_check: String,      // "Signature verification"
    pub check_passed: Option<bool>, // true/false/unknown
}

pub fn parse_snapshots_to_frames(
    snapshots: Vec<StepSnapshot>,
    source_map: &BTreeMap<u64, String>,
) -> Vec<Frame> {
    snapshots.into_iter().map(|snap| {
        Frame {
            step: snap.step,
            state_type: snap.state_type,
            
            // Convert UPLC to human language
            human_description: interpret_term_semantically(&snap.term),
            technical_detail: snap.term,
            
            validation_phase: infer_phase(&snap),
            current_check: infer_check(&snap),
            check_passed: detect_pass_fail(&snap),
            
            // ... rest of fields
        }
    }).collect()
}

fn interpret_term_semantically(term: &str) -> String {
    // Pattern matching on UPLC structures
    if term.contains("equalsInteger") {
        "Comparing two integers for equality"
    } else if term.contains("verifyEd25519Signature") {
        "Verifying cryptographic signature"
    } else if term.contains("lessThanInteger") {
        "Checking if value is below threshold"
    }
    // ... many more patterns
    .to_string()
}