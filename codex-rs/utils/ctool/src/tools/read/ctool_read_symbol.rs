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
use crate::tools::read::symbol_support::read_lines_range;

pub const CTOOL_READ_SYMBOL_TOOL_NAME: &str = "ctool_read_symbol";

const MAX_READ_SYMBOL_FILE_BYTES: u64 = 4 * 1024 * 1024;
const MAX_READ_SYMBOL_LINES: usize = 800;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolReadSymbolInput {
    pub path: PathBuf,
    pub symbol: String,
    #[serde(default)]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolReadSymbolOutput {
    pub path: String,
    pub symbol: String,
    pub kind: String,
    pub start_line: usize,
    pub end_line: usize,
    pub total_lines: usize,
    pub signature: String,
    pub content: String,
}

pub struct CToolReadSymbol;

impl CTool for CToolReadSymbol {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_READ_SYMBOL_TOOL_NAME,
            description: "Read one named Rust-like symbol range from a UTF-8 source file inside CToolScopeBase.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolReadSymbolInput = serde_json::from_value(input)?;
        let output = read_symbol(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

pub fn read_symbol(
    ctx: &CToolContext,
    input: CToolReadSymbolInput,
) -> CToolResult<CToolReadSymbolOutput> {
    let requested_symbol = input.symbol.trim();
    if requested_symbol.is_empty() {
        return Err(CToolError::InvalidInput(
            "symbol must not be empty".to_string(),
        ));
    }

    let path = gate::ensure_read_allowed(ctx, &input.path)?;
    let metadata = std::fs::metadata(&path)?;
    if !metadata.is_file() {
        return Err(CToolError::InvalidInput(format!(
            "path is not a file: {}",
            path.display()
        )));
    }
    if metadata.len() > MAX_READ_SYMBOL_FILE_BYTES {
        return Err(CToolError::InvalidInput(format!(
            "file is too large for read_symbol: {} bytes; max bytes: {}",
            metadata.len(),
            MAX_READ_SYMBOL_FILE_BYTES
        )));
    }

    let text = std::fs::read_to_string(&path).map_err(|error| {
        CToolError::InvalidInput(format!(
            "file is not valid UTF-8 text: {} ({error})",
            path.display()
        ))
    })?;
    let total_lines = text.lines().count();
    let requested_kind = input
        .kind
        .as_deref()
        .map(str::trim)
        .filter(|kind| !kind.is_empty());
    let matches = outline_symbols(&text)
        .into_iter()
        .filter(|symbol| symbol.name == requested_symbol)
        .filter(|symbol| requested_kind.map_or(true, |kind| symbol.kind == kind))
        .collect::<Vec<_>>();

    if matches.is_empty() {
        return Err(CToolError::InvalidInput(format!(
            "symbol not found: {requested_symbol}"
        )));
    }
    if matches.len() > 1 {
        let candidates = matches
            .iter()
            .map(|symbol| {
                format!(
                    "{} {}:{}-{}",
                    symbol.kind, symbol.name, symbol.start_line, symbol.end_line
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        return Err(CToolError::InvalidInput(format!(
            "symbol is ambiguous; provide kind. Candidates: {candidates}"
        )));
    }

    let symbol = matches.into_iter().next().expect("one match checked above");
    let line_count = symbol.end_line - symbol.start_line + 1;
    if line_count > MAX_READ_SYMBOL_LINES {
        return Err(CToolError::InvalidInput(format!(
            "symbol range is too large for read_symbol: {line_count} lines; max lines: {MAX_READ_SYMBOL_LINES}"
        )));
    }
    let content = read_lines_range(&text, symbol.start_line, symbol.end_line)?;

    Ok(CToolReadSymbolOutput {
        path: path.display().to_string(),
        symbol: symbol.name,
        kind: symbol.kind,
        start_line: symbol.start_line,
        end_line: symbol.end_line,
        total_lines,
        signature: symbol.signature,
        content,
    })
}
