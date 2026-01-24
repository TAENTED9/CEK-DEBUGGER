pub struct FailureReport {
    pub validator_intent: String,
    pub transaction_provided: Vec<String>,
    pub failure_reason: String,
    pub technical_root_cause: String,
    pub how_to_fix: Vec<String>,
    pub failing_code_snippet: Option<CodeSnippet>,
}

#[derive(Debug)]
pub struct CodeSnippet {
    pub source_file: String,
    pub line_number: usize,
    pub code: String,
    pub highlight: String,
}

impl FailureReport {
    pub fn generate(frames: &[Frame], failure: ValidationFailure) -> Self {
        FailureReport {
            validator_intent: failure.intent,
            transaction_provided: vec![
                format!("Redeemer: {}", failure.provided.redeemer),
                format!("Datum: {}", failure.provided.datum),
                format!("Signatories: {:?}", failure.provided.signatories),
            ],
            failure_reason: format_failure_reason(&failure.root_cause),
            technical_root_cause: extract_failing_term(frames),
            how_to_fix: failure.fix_suggestion.lines()
                .map(|s| s.to_string())
                .collect(),
            failing_code_snippet: find_source_location(frames),
        }
    }
    
    pub fn display(&self) -> String {
        format!(
            r#"
╔════════════════════════════════════════════════════════════╗
║              VALIDATOR FAILURE ANALYSIS                    ║
╠════════════════════════════════════════════════════════════╣

Validator Intent:
  {}

What Transaction Provided:
{}

Why Validation Failed:
  {}

Technical Root Cause:
  {}

How to Fix:
{}

{}
╚════════════════════════════════════════════════════════════╝
            "#,
            self.validator_intent,
            self.transaction_provided.iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            self.failure_reason,
            self.technical_root_cause,
            self.how_to_fix.iter()
                .enumerate()
                .map(|(i, step)| format!("  {}. {}", i + 1, step))
                .collect::<Vec<_>>()
                .join("\n"),
            self.failing_code_snippet.as_ref()
                .map(|s| format!(
                    "Failing Code ({}:L{}):\n  {}\n  ↑ {}",
                    s.source_file, s.line_number, s.code, s.highlight
                ))
                .unwrap_or_default()
        )
    }
}
```

---

## Implementation Priority

**Week 1: Foundation**
1. Add `ScriptContextSnapshot` to `StepSnapshot`
2. Implement basic pattern matching in `interpret_term_semantically()`
3. Create simple failure detection

**Week 2: Analysis**
1. Build `ValidatorAnalyzer` with common patterns
2. Implement `reconstruct_intent()` for top 10 validator types
3. Create mismatch detection logic

**Week 3: Reporting**
1. Build `FailureReport` generator
2. Create pretty-print formatter
3. Add source code snippet extraction

**Week 4: Polish**
1. Add more validator patterns
2. Improve fix suggestions
3. Create documentation

---

## Example Output (Target)
```
╔════════════════════════════════════════════════════════════╗
║              VALIDATOR FAILURE ANALYSIS                    ║
╠════════════════════════════════════════════════════════════╣

Validator Intent:
  This validator ensures the transaction is signed by the owner
  and the output value is at least 100 ADA.

What Transaction Provided:
  • Redeemer: Unit
  • Signatories: [addr1_test_xyz...]
  • Output Value: 50 ADA

Why Validation Failed:
  The output value (50 ADA) is below the required minimum (100 ADA).

Technical Root Cause:
  At step 47: lessThanInteger check returned False
  Expected: outputValue >= 100000000
  Actual:   outputValue == 50000000

How to Fix:
  1. Increase the output UTXO value to at least 100 ADA
  2. Or modify the validator to accept lower amounts
  3. Check your transaction builder configuration

Failing Code (vesting.ak:L23):
  if value >= minAmount then True else False
  ↑ This check failed because 50 ADA < 100 ADA
╚════════════════════════════════════════════════════════════╝