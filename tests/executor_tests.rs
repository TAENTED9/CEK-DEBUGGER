// tests/executor_tests.rs
use cek_debugger::executor::execute_program;
use uplc::ast::Program;
use uplc::ast::NamedDeBruijn;
use uplc::parser;

#[test]
fn test_simple_integer_program() {
    let program = parser::program("(program 1.0.0 (con integer 42))").unwrap();
    let program: Program<NamedDeBruijn> = program.try_into().unwrap();
    let snapshots = execute_program(program).unwrap();
    assert!(!snapshots.is_empty());
}

#[test]
fn test_lambda_application() {
    let _code = "(program 1.0.0 [(lam x x) (con integer 5)])";
    // ... test execution
}