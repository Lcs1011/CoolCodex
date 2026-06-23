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
use crate::tools::read::symbol_support::read_lines_range;

pub const CTOOL_READ_MANY_RANGES_TOOL_NAME: &str = "ctool_read_many_ranges";

const MAX_READ_MANY_RANGES_FILE_BYTES: u64 = 2 * 1024 * 1024;
const MAX_READ_MANY_RANGES_COUNT: usize = 20;
const MAX_READ_MANY_RANGES_TOTAL_LINES: usize = 800;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolReadManyRangesInput {
    pub path: PathBuf,
    pub ranges: Vec<CToolReadManyRangesRangeInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolReadManyRangesRangeInput {
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolReadManyRangesRangeOutput {
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolReadManyRangesOutput {
    pub path: String,
    pub total_lines: usize,
    pub range_count: usize,
    pub ranges: Vec<CToolReadManyRangesRangeOutput>,
}

pub struct CToolReadManyRanges;

impl CTool for CToolReadManyRanges {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_READ_MANY_RANGES_TOOL_NAME,
            description: "Read multiple inclusive line ranges from one UTF-8 text file inside CToolScopeBase.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolReadManyRangesInput = serde_json::from_value(input)?;
        let output = read_many_ranges(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

pub fn read_many_ranges(
    ctx: &CToolContext,
    input: CToolReadManyRangesInput,
) -> CToolResult<CToolReadManyRangesOutput> {
    if input.ranges.is_empty() {
        return Err(CToolError::InvalidInput(
            "ranges must not be empty".to_string(),
        ));
    }
    if input.ranges.len() > MAX_READ_MANY_RANGES_COUNT {
        return Err(CToolError::InvalidInput(format!(
            "read_many_ranges can read at most {MAX_READ_MANY_RANGES_COUNT} ranges"
        )));
    }

    let mut requested_total_lines = 0usize;
    for range in &input.ranges {
        if range.start_line == 0 {
            return Err(CToolError::InvalidInput(
                "start_line must be greater than 0".to_string(),
            ));
        }
        if range.end_line < range.start_line {
            return Err(CToolError::InvalidInput(
                "end_line must be greater than or equal to start_line".to_string(),
            ));
        }
        requested_total_lines += range.end_line - range.start_line + 1;
    }
    if requested_total_lines > MAX_READ_MANY_RANGES_TOTAL_LINES {
        return Err(CToolError::InvalidInput(format!(
            "read_many_ranges can read at most {MAX_READ_MANY_RANGES_TOTAL_LINES} total requested lines"
        )));
    }

    let path = gate::ensure_read_allowed(ctx, &input.path)?;
    let metadata = std::fs::metadata(&path)?;
    if !metadata.is_file() {
        return Err(CToolError::InvalidInput(format!(
            "path is not a file: {}",
            path.display()
        )));
    }
    if metadata.len() > MAX_READ_MANY_RANGES_FILE_BYTES {
        return Err(CToolError::InvalidInput(format!(
            "file is too large for read_many_ranges: {} bytes; max bytes: {}",
            metadata.len(),
            MAX_READ_MANY_RANGES_FILE_BYTES
        )));
    }

    let text = std::fs::read_to_string(&path).map_err(|error| {
        CToolError::InvalidInput(format!(
            "file is not valid UTF-8 text: {} ({error})",
            path.display()
        ))
    })?;
    let total_lines = text.lines().count();
    let ranges = input
        .ranges
        .into_iter()
        .map(|range| {
            if range.start_line > total_lines {
                return Err(CToolError::InvalidInput(format!(
                    "start_line {} is greater than total line count {}",
                    range.start_line, total_lines
                )));
            }
            if range.end_line > total_lines {
                return Err(CToolError::InvalidInput(format!(
                    "end_line {} is greater than total line count {}",
                    range.end_line, total_lines
                )));
            }
            let content = read_lines_range(&text, range.start_line, range.end_line)?;
            Ok(CToolReadManyRangesRangeOutput {
                start_line: range.start_line,
                end_line: range.end_line,
                content,
            })
        })
        .collect::<CToolResult<Vec<_>>>()?;

    Ok(CToolReadManyRangesOutput {
        path: path.display().to_string(),
        total_lines,
        range_count: ranges.len(),
        ranges,
    })
}
