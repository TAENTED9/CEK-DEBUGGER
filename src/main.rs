use std::{env, io::{self, Write}, path::PathBuf, time::Instant};
use anyhow::{Result, anyhow};

mod loader;
mod executor;
mod frames;
mod diagnostics;

use diagnostics::print_diagnostic;

fn print_frame(f: &frames::Frame, previous: Option<&frames::Frame>) {
    println!("\n{}", "═".repeat(80));
    println!("Step {:04} │ {} │ CPU: {:>10} │ MEM: {:>10}", 
        f.step, 
        format!("{:<10}", f.state_type),
        f.cpu,
        f.mem
    );
    println!("{}", "─".repeat(80));

    println!("📋 Term:\n{}\n", f.term);
    
    if !f.environment.is_empty() {
        println!("📦 Environment ({} bindings):", f.environment.len());
        for binding in f.environment.iter().take(5) {
            println!("   {}", binding);
        }
        if f.environment.len() > 5 {
            println!("   ... and {} more", f.environment.len() - 5);
        }
    }
    
    if f.context_depth > 0 {
        println!("🔀 Continuation Depth: {}", f.context_depth);
    }

    // Add smart diagnostics
    if let Some(diag) = diagnostics::analyze_frame(f, previous) {
        print_diagnostic(&diag);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <script.uplc|script.json> [param_hex ...]");
        return Ok(());
    }
    
    let path = PathBuf::from(&args[1]);
    let params = args[2..].to_vec();

    // Load program
    let programs = loader::load_programs_from_file(&path).await?;
    let mut program = programs.into_iter().next()
        .ok_or_else(|| anyhow!("No valid program found in file"))?;

    // Apply parameters
    let parsed_params = params.iter().enumerate()
        .map(|(i, p)| loader::parse_parameter(i, p.clone()))
        .collect::<Result<Vec<_>>>()?;
    
    program = loader::apply_parameters(program, parsed_params)?;

    // Execute with debugging
    let start = Instant::now();
    let snapshots = executor::execute_program(program.program)?;
    let duration = start.elapsed();
    println!("Execution took: {:?}", duration);
    
    let frames = frames::parse_snapshots_to_frames(snapshots, &program.source_map);

    println!("\n CEK Machine Debugger - {} steps captured\n", frames.len());

    // Interactive stepper
    let mut idx = 0;
    loop {
        if let Some(frame) = frames.get(idx) {
            let prev = if idx > 0 { frames.get(idx - 1) } else { None };
            print_frame(frame, prev);
        } else {
            println!("Invalid frame index");
            break;
        }

        print!("\n[N]ext | [P]rev | [J]ump | [Q]uit > ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().to_uppercase().as_str() {
            "N" => {
                if idx + 1 < frames.len() {
                    idx += 1;
                } else {
                    println!("⚠ At last step");
                }
            }
            "P" => {
                if idx > 0 {
                    idx -= 1;
                } else {
                    println!("⚠ At first step");
                }
            }
            "J" => {
                print!("Jump to step: ");
                io::stdout().flush()?;
                let mut jump = String::new();
                io::stdin().read_line(&mut jump)?;
                if let Ok(n) = jump.trim().parse::<usize>() {
                    if n < frames.len() {
                        idx = n;
                    } else {
                        println!("⚠ Invalid step (max: {})", frames.len() - 1);
                    }
                }
            }
            "Q" => break,
            _ => println!("⚠ Unknown command"),
        }
    }

    Ok(())
}