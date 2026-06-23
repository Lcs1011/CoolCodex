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
use crate::tools::edit::ctool_edit_exact::apply_insert_after_exact_to_text;
use crate::tools::edit::ctool_edit_exact::apply_insert_before_exact_to_text;
use crate::tools::edit::ctool_edit_exact::apply_remove_exact_to_text;
use crate::tools::edit::ctool_edit_exact::apply_replace_exact_to_text;
use crate::tools::edit::ctool_edit_insert::apply_insert_after_line_to_text;
use crate::tools::edit::ctool_edit_replace::apply_exact_replace_to_text;
use crate::tools::edit::ctool_preview_diff::make_simple_diff;
use crate::tools::edit::ensure_editable_text_file_size;

pub const CTOOL_EDIT_PREVIEW_TOOL_NAME: &str = "ctool_edit_preview";

const MAX_EDIT_PREVIEW_OPERATIONS: usize = 50;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolEditPreviewInput {
    pub path: PathBuf,
    pub operations: Vec<CToolEditPreviewOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum CToolEditPreviewOperation {
    Replace {
        old_string: String,
        new_string: String,
    },
    Insert {
        insert_after_line: usize,
        content: String,
    },
    ReplaceExact {
        anchor: String,
        target: String,
        content: String,
    },
    InsertBeforeExact {
        anchor: String,
        target: String,
        content: String,
    },
    InsertAfterExact {
        anchor: String,
        target: String,
        content: String,
    },
    RemoveExact {
        anchor: String,
        target: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolEditPreviewOutput {
    pub path: String,
    pub operation_count: usize,
    pub changed: bool,
    pub diff: String,
}

pub struct CToolEditPreview;

impl CTool for CToolEditPreview {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_EDIT_PREVIEW_TOOL_NAME,
            description: "Preview exact/line edit operations for one UTF-8 file without writing it.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolEditPreviewInput = serde_json::from_value(input)?;
        let output = edit_preview(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

pub fn edit_preview(
    ctx: &CToolContext,
    input: CToolEditPreviewInput,
) -> CToolResult<CToolEditPreviewOutput> {
    if input.operations.is_empty() {
        return Err(CToolError::InvalidInput(
            "operations must not be empty".to_string(),
        ));
    }
    if input.operations.len() > MAX_EDIT_PREVIEW_OPERATIONS {
        return Err(CToolError::InvalidInput(format!(
            "edit_preview can preview at most {MAX_EDIT_PREVIEW_OPERATIONS} operations"
        )));
    }

    let path = gate::ensure_write_allowed(ctx, &input.path)?;
    ensure_editable_text_file_size(&path, "edit_preview")?;

    let before = std::fs::read_to_string(&path)?;
    let mut after = before.clone();

    for operation in &input.operations {
        after = match operation {
            CToolEditPreviewOperation::Replace {
                old_string,
                new_string,
            } => apply_exact_replace_to_text(&after, old_string, new_string)?,
            CToolEditPreviewOperation::Insert {
                insert_after_line,
                content,
            } => apply_insert_after_line_to_text(&after, *insert_after_line, content)?,
            CToolEditPreviewOperation::ReplaceExact {
                anchor,
                target,
                content,
            } => apply_replace_exact_to_text(&after, anchor, target, content)?,
            CToolEditPreviewOperation::InsertBeforeExact {
                anchor,
                target,
                content,
            } => apply_insert_before_exact_to_text(&after, anchor, target, content)?,
            CToolEditPreviewOperation::InsertAfterExact {
                anchor,
                target,
                content,
            } => apply_insert_after_exact_to_text(&after, anchor, target, content)?,
            CToolEditPreviewOperation::RemoveExact { anchor, target } => {
                apply_remove_exact_to_text(&after, anchor, target)?
            }
        };
    }

    Ok(CToolEditPreviewOutput {
        path: path.display().to_string(),
        operation_count: input.operations.len(),
        changed: before != after,
        diff: make_simple_diff(&before, &after),
    })
}
