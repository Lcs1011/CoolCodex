pub mod ctool_git_diff_file;
pub mod ctool_git_diff_summary;

use std::path::Path;
use std::process::Command;

use crate::error::CToolError;
use crate::error::CToolResult;

const MAX_GIT_OUTPUT_BYTES: usize = 200_000;

pub use ctool_git_diff_file::CTOOL_GIT_DIFF_FILE_TOOL_NAME;
pub use ctool_git_diff_file::CToolGitDiffFile;
pub use ctool_git_diff_file::CToolGitDiffFileInput;
pub use ctool_git_diff_file::CToolGitDiffFileOutput;
pub use ctool_git_diff_summary::CTOOL_GIT_DIFF_SUMMARY_TOOL_NAME;
pub use ctool_git_diff_summary::CToolGitDiffSummary;
pub use ctool_git_diff_summary::CToolGitDiffSummaryInput;
pub use ctool_git_diff_summary::CToolGitDiffSummaryOutput;

pub(crate) fn run_git(work_dir: &Path, args: &[&str]) -> CToolResult<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(work_dir)
        .output()?;
    let mut text = String::new();
    if !output.stdout.is_empty() {
        text.push_str(&String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str(&String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        return Err(CToolError::InvalidInput(format!(
            "git {:?} failed with exit code {:?}: {}",
            args,
            output.status.code(),
            text
        )));
    }

    Ok(truncate_git_output(&text))
}

fn truncate_git_output(text: &str) -> String {
    if text.len() <= MAX_GIT_OUTPUT_BYTES {
        return text.to_string();
    }

    let mut output = text[..MAX_GIT_OUTPUT_BYTES].to_string();
    output.push_str("\n...[truncated]");
    output
}
