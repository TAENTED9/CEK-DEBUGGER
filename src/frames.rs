use serde::Serialize;
use uplc::machine::debug::StepSnapshot;
use std::collections::BTreeMap;

#[derive(Clone, Serialize)]
pub struct Frame {
    pub step: usize,
    pub state_type: String,
    pub term: String,
    pub environment: Vec<String>,
    pub context_depth: usize,
    pub cpu: i64,
    pub mem: i64,
    pub source_location: Option<String>,
}

pub fn parse_snapshots_to_frames(
    snapshots: Vec<StepSnapshot>,
    source_map: &BTreeMap<u64, String>,
) -> Vec<Frame> {
    snapshots.into_iter().map(|snap| {
        Frame {
            step: snap.step,
            state_type: snap.state_type,
            term: snap.term,
            environment: snap.environment,
            context_depth: snap.context_depth,
            cpu: snap.cpu,
            mem: snap.mem,
            source_location: source_map.get(&(snap.step as u64)).cloned(),
        }
    }).collect()
}