use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::context::CToolContext;
use crate::error::CToolError;
use crate::error::CToolResult;
use crate::gate;
use crate::tool::CTool;
use crate::tool::CToolSpec;
use crate::tools::read::symbol_support::outline_symbols;

pub const CTOOL_OUTLINE_FILE_TOOL_NAME: &str = "ctool_outline_file";

const MAX_OUTLINE_FILE_BYTES: u64 = 4 * 1024 * 1024;
const MAX_OUTLINE_SYMBOLS: usize = 400;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolOutlineFileInput {
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolOutlineSymbol {
    pub kind: String,
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolOutlineFileOutput {
    pub path: String,
    pub total_lines: usize,
    pub symbol_count: usize,
    pub truncated: bool,
    pub symbols: Vec<CToolOutlineSymbol>,
}

pub struct CToolOutlineFile;

impl CTool for CToolOutlineFile {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_OUTLINE_FILE_TOOL_NAME,
            description: "Read a UTF-8 source file outline without returning the full file content.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolOutlineFileInput = serde_json::from_value(input)?;
        let output = outline_file(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

pub fn outline_file(
    ctx: &CToolContext,
    input: CToolOutlineFileInput,
) -> CToolResult<CToolOutlineFileOutput> {
    let path = gate::ensure_read_allowed(ctx, &input.path)?;
    let metadata = std::fs::metadata(&path)?;
    if !metadata.is_file() {
        return Err(CToolError::InvalidInput(format!(
            "path is not a file: {}",
            path.display()
        )));
    }
    if metadata.len() > MAX_OUTLINE_FILE_BYTES {
        return Err(CToolError::InvalidInput(format!(
            "file is too large for outline_file: {} bytes; max bytes: {}",
            metadata.len(),
            MAX_OUTLINE_FILE_BYTES
        )));
    }

    let text = std::fs::read_to_string(&path).map_err(|error| {
        CToolError::InvalidInput(format!(
            "file is not valid UTF-8 text: {} ({error})",
            path.display()
        ))
    })?;
    let total_lines = text.lines().count();
    let mut symbols = outline_symbols(&text)
        .into_iter()
        .map(|symbol| CToolOutlineSymbol {
            kind: symbol.kind,
            name: symbol.name,
            start_line: symbol.start_line,
            end_line: symbol.end_line,
            signature: symbol.signature,
        })
        .collect::<Vec<_>>();
    let truncated = symbols.len() > MAX_OUTLINE_SYMBOLS;
    symbols.truncate(MAX_OUTLINE_SYMBOLS);

    Ok(CToolOutlineFileOutput {
        path: path.display().to_string(),
        total_lines,
        symbol_count: symbols.len(),
        truncated,
        symbols,
    })
}
