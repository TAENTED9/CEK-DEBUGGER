use std::{env, io::{self, Write}, path::PathBuf};
use anyhow::Result;

mod loader;
mod executor;
mod frames;

fn print_frame(f: &frames::Frame) {
    println!("\n{}", "=".repeat(60));
    println!("Step {}: {}", f.step, f.state_type);
    println!("{}", "=".repeat(60));
    println!("Term: {}", f.term);
    
    if !f.environment.is_empty() {
        println!("\nEnvironment ({} bindings):", f.environment.len());
        for binding in &f.environment {
            println!("  {}", binding);
        }
    }
    
    if f.context_depth > 0 {
        println!("\nContinuation depth: {}", f.context_depth);
    }
    
    if let Some(loc) = &f.source_location {
        println!("\nSource: {}", loc);
    }
    
    println!("\nBudget: CPU={} MEM={}", f.cpu, f.mem);
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
    let mut program = programs.into_iter().next().expect("no program");
    
    // Apply parameters
    let parsed_params = params.iter().enumerate()
        .map(|(i, p)| loader::parse_parameter(i, p.clone()))
        .collect::<Result<Vec<_>>>()?;
    
    program = loader::apply_parameters(program, parsed_params)?;

    // Execute with debugging
    let snapshots = executor::execute_program(program.program)?;
    let frames = frames::parse_snapshots_to_frames(snapshots, &program.source_map);

    println!("\n🔍 CEK Machine Debugger - {} steps captured\n", frames.len());

    // Interactive stepper
    let mut idx = 0;
    loop {
        print_frame(&frames[idx]);

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