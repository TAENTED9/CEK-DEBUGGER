use std::{collections::BTreeMap, ffi::OsStr, fs, path::Path};

use anyhow::{Context, Result, anyhow};
use minicbor::bytes::ByteVec;
use serde::Deserialize;
pub use uplc::ast::Program;
use uplc::{
    PlutusData,  // Removed Fragment - it's unused
    ast::{DeBruijn, FakeNamedDeBruijn, Name, NamedDeBruijn},
    parser,
};

#[derive(Clone)]
pub struct LoadedProgram {
    pub filename: String,
    pub program: Program<NamedDeBruijn>,
    pub source_map: BTreeMap<u64, String>,
}

enum FileType {
    Uplc,
    Flat,
    Json,
}

fn identify_file_type(file: &Path) -> Result<FileType> {
    let extension = file.extension().and_then(OsStr::to_str);
    match extension {
        Some("uplc") => Ok(FileType::Uplc),
        Some("flat") => Ok(FileType::Flat),
        Some("json") => Ok(FileType::Json),
        _ => Err(anyhow!("That extension is not supported. Supported: .uplc .flat .json")),
    }
}

fn fix_names(program: Program<NamedDeBruijn>) -> Result<Program<NamedDeBruijn>> {
    let debruijn: Program<DeBruijn> = program.into();
    let name: Program<Name> = debruijn.try_into()?;
    let named_de_bruijn: Program<NamedDeBruijn> = name.try_into()?;
    Ok(named_de_bruijn)
}

fn load_flat(bytes: &[u8]) -> Result<Program<NamedDeBruijn>> {
    let fake_named_de_bruijn: Program<FakeNamedDeBruijn> = Program::from_flat(bytes)?;
    Ok(fake_named_de_bruijn.into())
}

pub async fn load_programs_from_file(file: &Path) -> Result<Vec<LoadedProgram>> {
    let filename = file.display().to_string();
    match identify_file_type(file)? {
        FileType::Uplc => {
            let code = fs::read_to_string(file)?;
            let program = parser::program(&code)
                .context("parser::program failed")?
                .try_into()?;
            let source_map = BTreeMap::new();
            Ok(vec![LoadedProgram { filename, program, source_map }])
        }
        FileType::Flat => {
            let bytes = std::fs::read(file)?;
            let program = fix_names(load_flat(&bytes)?)?;
            let source_map = BTreeMap::new();
            Ok(vec![LoadedProgram { filename, program, source_map }])
        }
        FileType::Json => {
            let raw = fs::read(file).context("could not read json file")?;
            let export: AikenExport = serde_json::from_slice(&raw)
                .context("could not parse aiken json")?;

            let bytes = hex::decode(&export.compiled_code)
                .context("could not hex-decode compiled_code")?;

            let cbor: ByteVec = minicbor::decode(&bytes)
                .map_err(|e| anyhow!("minicbor decode failed: {:?}", e))?;

            let inner: Vec<u8> = cbor.into();
            let program = fix_names(load_flat(&inner)?)?;
            let source_map = export.source_map.unwrap_or_default();
            Ok(vec![LoadedProgram { filename, program, source_map }])
        }
    }
}

pub fn parse_parameter(index: usize, parameter: String) -> Result<PlutusData> {
    let bytes = hex::decode(&parameter)
        .context(format!("could not hex-decode parameter {}", index))?;
    let data = uplc::plutus_data(&bytes)
        .map_err(|e| anyhow!("could not decode plutus data for parameter {}: {}", index, e))?;
    Ok(data)
}

pub fn apply_parameters(
    LoadedProgram { filename, program, source_map }: LoadedProgram,
    parameters: Vec<PlutusData>,
) -> Result<LoadedProgram> {
    let mut program = program;
    let mut source_map_offset = 0u64;
    for param in parameters {
        program = program.apply_data(param);
        source_map_offset += 1;
    }
    let source_map = source_map
        .into_iter()
        .map(|(index, location)| (index + source_map_offset, location))
        .collect();
    Ok(LoadedProgram { filename, program, source_map })
}

#[derive(Deserialize, Debug)]  // Added Deserialize derive
#[serde(rename_all = "camelCase")]
struct AikenExport {
    compiled_code: String,
    source_map: Option<BTreeMap<u64, String>>,
}