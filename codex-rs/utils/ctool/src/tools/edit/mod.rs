pub mod ctool_edit_batch;
pub mod ctool_edit_insert;
pub mod ctool_edit_replace;
pub mod ctool_preview_diff;

use std::path::Path;

use crate::error::CToolError;
use crate::error::CToolResult;

const MAX_EDIT_FILE_BYTES: u64 = 2 * 1024 * 1024;

pub(crate) fn ensure_editable_text_file_size(path: &Path, operation: &str) -> CToolResult<()> {
    let metadata = std::fs::metadata(path)?;
    if !metadata.is_file() {
        return Err(CToolError::InvalidInput(format!(
            "{operation} requires a file: {}",
            path.display()
        )));
    }
    if metadata.len() > MAX_EDIT_FILE_BYTES {
        return Err(CToolError::InvalidInput(format!(
            "file is too large for {operation}: {} bytes; max bytes: {}",
            metadata.len(),
            MAX_EDIT_FILE_BYTES
        )));
    }

    Ok(())
}

pub use ctool_edit_replace::CTOOL_EDIT_REPLACE_TOOL_NAME;
pub use ctool_edit_replace::CToolEditReplace;
pub use ctool_edit_replace::CToolEditReplaceInput;
pub use ctool_edit_replace::CToolEditReplaceOutput;

pub use ctool_edit_insert::CTOOL_EDIT_INSERT_TOOL_NAME;
pub use ctool_edit_insert::CToolEditInsert;
pub use ctool_edit_insert::CToolEditInsertInput;
pub use ctool_edit_insert::CToolEditInsertOutput;

pub use ctool_preview_diff::CTOOL_PREVIEW_DIFF_TOOL_NAME;
pub use ctool_preview_diff::CToolPreviewDiff;
pub use ctool_preview_diff::CToolPreviewDiffInput;
pub use ctool_preview_diff::CToolPreviewDiffOperation;
pub use ctool_preview_diff::CToolPreviewDiffOutput;

pub use ctool_edit_batch::CTOOL_EDIT_BATCH_TOOL_NAME;
pub use ctool_edit_batch::CToolEditBatch;
pub use ctool_edit_batch::CToolEditBatchInput;
pub use ctool_edit_batch::CToolEditBatchOperation;
pub use ctool_edit_batch::CToolEditBatchOutput;
