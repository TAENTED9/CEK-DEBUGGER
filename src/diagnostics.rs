/// Human-readable diagnostic messages for validators
/// Translates CEK machine states into plain English debugging insights

use crate::frames::Frame;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub title: String,
    pub status: DiagnosticStatus,
    pub explanation: String,
    pub next_steps: Vec<String>,
    pub severity: Severity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticStatus {
    Info,
    Warning,
    Error,
    Success,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Severity {
    Debug,    // Information only
    Hint,     // Helpful suggestion
    Warning,  // Something might be wrong
    Error,    // Validation failed
    Critical, // Fatal error
}

impl Severity {
    pub fn symbol(&self) -> &'static str {
        match self {
            Severity::Debug => "ℹ️",
            Severity::Hint => "💡",
            Severity::Warning => "⚠️",
            Severity::Error => "❌",
            Severity::Critical => "🚨",
        }
    }
}

impl DiagnosticStatus {
    pub fn symbol(&self) -> &'static str {
        match self {
            DiagnosticStatus::Info => "ℹ",
            DiagnosticStatus::Warning => "⚠",
            DiagnosticStatus::Error => "✗",
            DiagnosticStatus::Success => "✓",
        }
    }
}

pub fn analyze_frame(frame: &Frame, _previous_frame: Option<&Frame>) -> Option<Diagnostic> {
    let term_lower = frame.term.to_lowercase();

    // Check for error states
    if term_lower.contains("error") {
        return Some(diagnose_error(&frame.term));
    }

    // Check for missing signatures
    if term_lower.contains("unConstrData") || term_lower.contains("signature") {
        return Some(Diagnostic {
            title: "Checking Signature Validation".to_string(),
            status: DiagnosticStatus::Info,
            explanation: "The validator is checking if data has been properly signed. Make sure:\n  • Redeemer is correctly signed with the validator's key\n  • Datum contains the expected signature".to_string(),
            next_steps: vec![
                "Verify you're using the correct signing key".to_string(),
                "Check that the transaction signature matches the validator expectations".to_string(),
                "Ensure datum contains all required signature fields".to_string(),
            ],
            severity: Severity::Hint,
        });
    }

    // Check for signature comparison
    if term_lower.contains("equalsByteString") {
        return Some(Diagnostic {
            title: "Comparing Signatures or Data".to_string(),
            status: DiagnosticStatus::Info,
            explanation: "The validator is comparing byte strings - likely checking:\n  • Cryptographic signatures match\n  • Public keys are valid\n  • Hashes are identical".to_string(),
            next_steps: vec![
                "If this fails, signatures don't match".to_string(),
                "Check that both signatures use the same key".to_string(),
                "Verify no data was modified between signing and validation".to_string(),
            ],
            severity: Severity::Warning,
        });
    }

    // Check for redeemer validation
    if term_lower.contains("unMapData") || term_lower.contains("unConstrData") {
        return Some(Diagnostic {
            title: "Processing Validator Input".to_string(),
            status: DiagnosticStatus::Info,
            explanation: "The validator is unpacking the redeemer or datum structure. This is where:\n  • Redeemer data is extracted from the transaction\n  • Datum constraints are read\n  • Parameters are prepared for validation logic".to_string(),
            next_steps: vec![
                "Ensure redeemer structure matches what validator expects".to_string(),
                "Check that datum contains all required fields".to_string(),
                "Verify data types match (integers, bytes, etc.)".to_string(),
            ],
            severity: Severity::Hint,
        });
    }

    // Check for boolean decisions
    if term_lower.contains("ifThenElse") {
        return Some(Diagnostic {
            title: "Validation Decision Point".to_string(),
            status: DiagnosticStatus::Info,
            explanation: "The validator is making a critical decision:\n  • If the condition is TRUE → ✓ Validation PASSES\n  • If the condition is FALSE → ✗ Validation FAILS\n\nThis is where your validator logic decides accept or reject.".to_string(),
            next_steps: vec![
                "Check if the condition matches your expectations".to_string(),
                "If it fails, your inputs don't satisfy the validator's rules".to_string(),
                "Review the logic before this decision".to_string(),
            ],
            severity: Severity::Warning,
        });
    }

    // Check for force/delay (lazy evaluation)
    if term_lower.contains("force") || term_lower.contains("delay") {
        return Some(Diagnostic {
            title: "Lazy Evaluation".to_string(),
            status: DiagnosticStatus::Info,
            explanation: "The validator is using lazy evaluation. Think of it like:\n  • DELAY = 'wait to calculate this'\n  • FORCE = 'calculate it now'\n\nThis helps validators avoid unnecessary computation.".to_string(),
            next_steps: vec![
                "No action needed - this is normal validator behavior".to_string(),
                "If stuck here, the validator may have circular dependencies".to_string(),
            ],
            severity: Severity::Debug,
        });
    }

    // Check for computations
    if term_lower.contains("builtin") {
        if term_lower.contains("addInteger") || term_lower.contains("subtractInteger") {
            return Some(Diagnostic {
                title: "Performing Math Calculation".to_string(),
                status: DiagnosticStatus::Info,
                explanation: "The validator is performing arithmetic. Make sure:\n  • Input numbers are in the correct range\n  • No integer overflow\n  • Result is as expected".to_string(),
                next_steps: vec![
                    "Check your input values".to_string(),
                    "Verify calculation matches your logic".to_string(),
                ],
                severity: Severity::Debug,
            });
        }

        if term_lower.contains("appendByteString") || term_lower.contains("sliceByteString") {
            return Some(Diagnostic {
                title: "Processing Byte Data".to_string(),
                status: DiagnosticStatus::Info,
                explanation: "The validator is manipulating byte strings:\n  • Concatenating data\n  • Slicing/extracting portions\n  • Building composite data structures".to_string(),
                next_steps: vec![
                    "Verify byte string lengths are correct".to_string(),
                    "Check slice indices are within bounds".to_string(),
                ],
                severity: Severity::Debug,
            });
        }
    }

    // Check for lambda (function) application
    if term_lower.contains("lam") {
        return Some(Diagnostic {
            title: "Executing Validator Function".to_string(),
            status: DiagnosticStatus::Info,
            explanation: "A validator function is being executed with parameters. The validator is:\n  • Receiving transaction data\n  • Processing your redeemer\n  • Applying validation rules".to_string(),
            next_steps: vec![
                "Continue stepping to see validation logic".to_string(),
                "Watch for decision points (ifThenElse)".to_string(),
            ],
            severity: Severity::Debug,
        });
    }

    // Check for constants (data literals)
    if term_lower.contains("con") {
        if term_lower.contains("unit") {
            return Some(Diagnostic {
                title: "Unit Value (Placeholder)".to_string(),
                status: DiagnosticStatus::Info,
                explanation: "Unit is like a placeholder or 'nothing' value in Plutus. Used when:\n  • A function returns nothing\n  • Placeholder in tuples\n  • Void/null equivalent".to_string(),
                next_steps: vec!["This is normal - continue execution".to_string()],
                severity: Severity::Debug,
            });
        }
    }

    // Final success
    if frame.step > 0 && term_lower.contains("con") && !term_lower.contains("error") {
        return Some(Diagnostic {
            title: "Execution Complete".to_string(),
            status: DiagnosticStatus::Success,
            explanation: "The validator has finished executing and produced a result:\n  • ✓ Validation logic completed\n  • ✓ All checks passed\n  • ✓ Result generated successfully".to_string(),
            next_steps: vec![
                "Validator accepted the transaction".to_string(),
                "Check result value above".to_string(),
            ],
            severity: Severity::Debug,
        });
    }

    None
}

fn diagnose_error(term: &str) -> Diagnostic {
    let term_lower = term.to_lowercase();

    if term_lower.contains("signature") {
        return Diagnostic {
            title: "❌ SIGNATURE VALIDATION FAILED".to_string(),
            status: DiagnosticStatus::Error,
            explanation: "The validator rejected because:\n  ✗ Signature doesn't match validator expectations\n  ✗ Signing key is incorrect\n  ✗ Data was modified after signing".to_string(),
            next_steps: vec![
                "1️⃣ Check you're using the correct key pair".to_string(),
                "2️⃣ Verify the signature matches the validator's public key".to_string(),
                "3️⃣ Ensure no data changed between signing and submission".to_string(),
                "4️⃣ Re-sign the transaction with the correct key".to_string(),
            ],
            severity: Severity::Critical,
        };
    }

    if term_lower.contains("redeemer") {
        return Diagnostic {
            title: "❌ INVALID REDEEMER".to_string(),
            status: DiagnosticStatus::Error,
            explanation: "The validator rejected because:\n  ✗ Redeemer format is incorrect\n  ✗ Redeemer doesn't contain required fields\n  ✗ Redeemer values don't match validator expectations".to_string(),
            next_steps: vec![
                "1️⃣ Check redeemer structure matches validator requirements".to_string(),
                "2️⃣ Verify all required fields are present".to_string(),
                "3️⃣ Check data types (integers, bytes, etc.)".to_string(),
                "4️⃣ Use correct redeemer for this endpoint".to_string(),
            ],
            severity: Severity::Critical,
        };
    }

    if term_lower.contains("datum") {
        return Diagnostic {
            title: "❌ INVALID DATUM".to_string(),
            status: DiagnosticStatus::Error,
            explanation: "The validator rejected because:\n  ✗ Datum is missing required fields\n  ✗ Datum data doesn't match expectations\n  ✗ Datum wasn't signed correctly".to_string(),
            next_steps: vec![
                "1️⃣ Check datum contains all required data".to_string(),
                "2️⃣ Verify datum structure matches UTXO specification".to_string(),
                "3️⃣ Ensure datum is properly encoded in the UTXO".to_string(),
                "4️⃣ Check transaction includes this UTXO".to_string(),
            ],
            severity: Severity::Critical,
        };
    }

    if term_lower.contains("equalsByteString") {
        return Diagnostic {
            title: "❌ DATA COMPARISON FAILED".to_string(),
            status: DiagnosticStatus::Error,
            explanation: "The validator rejected because:\n  ✗ Two pieces of data don't match\n  ✗ Usually: expected value ≠ actual value\n  ✗ Could be: signature, hash, or constraint mismatch".to_string(),
            next_steps: vec![
                "1️⃣ Compare the expected vs actual values".to_string(),
                "2️⃣ Check for typos in keys or addresses".to_string(),
                "3️⃣ Verify cryptographic hashes match".to_string(),
                "4️⃣ Review transaction data".to_string(),
            ],
            severity: Severity::Critical,
        };
    }

    if term_lower.contains("equalsInteger") {
        return Diagnostic {
            title: "❌ VALIDATION CHECK FAILED".to_string(),
            status: DiagnosticStatus::Error,
            explanation: "The validator rejected because:\n  ✗ Two numbers don't match\n  ✗ Usually: transaction amount, count, or ID mismatch\n  ✗ Constraint violation: expected X, got Y".to_string(),
            next_steps: vec![
                "1️⃣ Check transaction amounts/counts".to_string(),
                "2️⃣ Verify input values match validator expectations".to_string(),
                "3️⃣ Review numeric constraints".to_string(),
            ],
            severity: Severity::Critical,
        };
    }

    // Generic error
    Diagnostic {
        title: "❌ VALIDATOR REJECTION".to_string(),
        status: DiagnosticStatus::Error,
        explanation: format!("The validator rejected the transaction.\n\nError term: {}", term),
        next_steps: vec![
            "1️⃣ Review the error term above".to_string(),
            "2️⃣ Check all redeemer and datum fields".to_string(),
            "3️⃣ Verify cryptographic signatures".to_string(),
            "4️⃣ Ensure transaction data matches validator rules".to_string(),
        ],
        severity: Severity::Critical,
    }
}

pub fn print_diagnostic(diag: &Diagnostic) {
    println!("\n{}", "═".repeat(80));
    println!("{} {}  [{}]", 
        diag.severity.symbol(),
        diag.title,
        match diag.status {
            DiagnosticStatus::Info => "INFO",
            DiagnosticStatus::Warning => "WARNING",
            DiagnosticStatus::Error => "ERROR",
            DiagnosticStatus::Success => "SUCCESS",
        }
    );
    println!("{}", "─".repeat(80));

    println!("\n📝 {}\n", diag.explanation);

    if !diag.next_steps.is_empty() {
        println!("✅ What to do next:");
        for step in diag.next_steps.iter() {
            println!("   {}", step);
        }
    }

    println!("{}", "═".repeat(80));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_detection() {
        let frame = Frame {
            step: 0,
            state_type: "Error".to_string(),
            term: "(error \"invalid signature\")".to_string(),
            environment: vec![],
            context_depth: 0,
            cpu: 0,
            mem: 0,
            source_location: None,
        };

        let diag = analyze_frame(&frame, None);
        assert!(diag.is_some());
        assert_eq!(diag.unwrap().status, DiagnosticStatus::Error);
    }
}
