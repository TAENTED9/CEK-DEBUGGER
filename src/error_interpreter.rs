use uplc::machine::indexed_term::IndexedTerm;
use anyhow::Result;

/// Simple interpreter of machine errors (IndexedTerm::Error)
pub fn interpret_error(term: &IndexedTerm<u64>) -> Option<String> {
    match term {
        IndexedTerm::Error { index: _ } => Some("UPLC evaluation error (IndexedTerm::Error)".to_string()),
        _ => None
    }
}
