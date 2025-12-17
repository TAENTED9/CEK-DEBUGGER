use std::{env, io::{self, Write}, path::PathBuf};
use anyhow::Result;
use crate::{loader, executor, frames, error_interpreter};

fn print_frame(f: &frames::Frame) {
    println!("--- step {}/{} ---", f.index, "??");
    println!("label: {}", f.label);
    if let Some(loc) = &f.location {
        println!("source: {}", loc);
    }
    println!("term: {}", f.term);
    println!("context: {:?}", f.context);
    println!("env:");
    for (n, v) in &f.env {
        println!("  {} = {}", n, v);
    }
    println!("budget: steps={} mem={} (+{} / +{})", f.budget.steps, f.budget.mem, f.budget.steps_diff, f.budget.mem_diff);
    if let Some(rv) = &f.ret_value {
        println!("return value: {}", rv);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <script.uplc|script.json> [param_hex ...]");
        return Ok(());
    }
    let path = PathBuf::from(&args[1]);
    let params = args.drain(2..).collect::<Vec<_>>();

    // load program
    let raw_programs = loader::load_programs_from_file(&path).await?;
    let mut program = raw_programs.into_iter().next().expect("no program loaded");

    // parse params
    let parsed_params = params.into_iter().enumerate().map(|(i,p)| loader::parse_parameter(i, p)).collect::<Result<Vec<_>, _>>()?;
    let program = loader::apply_parameters(program, parsed_params)?;

    // execute
    let states = executor::execute_program(program.program)?;
    // parse frames
    let frames_vec = frames::parse_raw_frames(&states, &program.source_map);

    // simple CLI: step through frames
    let mut idx = 0usize;
    loop {
        let frame = &frames_vec[idx];
        print_frame(frame);

        // show if final done was an Error
        if let uplc::machine::indexed_term::IndexedTerm::Error { index: _ } = &states[idx].0.clone().into_done_term().unwrap_or(uplc::machine::indexed_term::IndexedTerm::Error { index: None }) {
            if let Some(msg) = error_interpreter::interpret_error(&uplc::machine::indexed_term::IndexedTerm::Error { index: None }) {
                println!("INTERPRETATION: {}", msg);
            }
        }

        print!("(N)ext, (P)rev, (Q)uit > ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_uppercase().as_str() {
            "N" => { if idx + 1 < frames_vec.len() { idx += 1 } else { println!("Already at last frame") } }
            "P" => { if idx > 0 { idx -= 1 } else { println!("Already at first frame") } }
            "Q" => break,
            other => println!("Unknown: {}", other),
        }
    }

    Ok(())
}
