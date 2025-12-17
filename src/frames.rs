use std::{collections::BTreeMap, fs, path::Path};
use serde::Serialize;
use uplc::ast::NamedDeBruijn;
use uplc::machine::{Trace};
use uplc::machine::cost_model::ExBudget;
use crate::loader::LoadedProgram;

#[derive(Clone, Serialize)]
pub struct Frame {
    pub index: usize,
    pub label: String,       // human readable trace line/label
    pub location: Option<String>, // optional source file:line from source_map
    pub budget: ExBudget,
    pub steps_diff: i64,
    pub mem_diff: i64,
}

// A minimal parser that converts machine traces into frames.
// Because the uplc crate hides MachineState, Context and Env, we cannot reconstruct the
// per-step term/env/context. We instead present the public trace lines (logs/labels)
// and per-trace budget snapshots. This still gives useful info (labels, logs, cost deltas, source mapping).
pub fn parse_raw_frames(
    traces: &[Trace],
    source_map: &BTreeMap<u64, String>,
    final_budget: &ExBudget,
) -> Vec<Frame> {
    let mut frames = Vec::new();

    // We don't have budget snapshots per-trace in the public API,
    // only the final ex_budget and the traces themselves.
    // For a useful approximation we will set the same budget on all frames,
    // and show diffs relative to first frame (zero) — this is conservative.
    let mut prev_steps = 0i64;
    let mut prev_mem = 0i64;

    for (i, trace) in traces.iter().enumerate() {
        // trace string form
        let label = trace.to_string();

        // Attempt to find a source location encoded in a label like "file:line"
        // (This is heuristic — Aiken/compilers often include such labels in trace logs.)
        let location = label.split_whitespace().find_map(|tok| {
            // detect common "file:line" form
            if tok.contains(':') && tok.ends_with(".ak") {
                // e.g. "validators/foo.ak:12"
                Some(tok.to_string())
            } else {
                None
            }
        });

        // Use provided final budget as a snapshot (best public info).
        // Convert fields to signed differences for the UI.
        let steps = final_budget.cpu as i64;
        let mem = final_budget.mem as i64;

        let frame = Frame {
            index: i,
            label,
            location,
            budget: final_budget.clone(),
            steps_diff: steps - prev_steps,
            mem_diff: mem - prev_mem,
        };

        prev_steps = steps;
        prev_mem = mem;

        frames.push(frame);
    }

    frames
}

// read source files for mapping (optional)
pub fn read_source_files(source_root: &Path, frames: &[Frame]) -> BTreeMap<String, String> {
    use std::collections::BTreeSet;
    let filenames: BTreeSet<&str> = frames.iter().filter_map(|f| f.location.as_ref()).filter_map(|loc| loc.split_once(":")).map(|(file,_)| file).collect();
    let mut roots = vec![source_root.join("validators"), source_root.join("lib")];
    let mut files = BTreeMap::new();
    for filename in filenames {
        if let Some(contents) = roots.iter().find_map(|root| fs::read_to_string(root.join(filename)).ok()) {
            files.insert(filename.to_string(), contents);
        }
    }
    files
}
