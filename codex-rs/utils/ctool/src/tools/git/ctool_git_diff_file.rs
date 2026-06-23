use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::context::CToolContext;
use crate::error::CToolError;
use crate::error::CToolResult;
use crate::scope_context::can_read_path;
use crate::scope_context::resolve_user_path;
use crate::tool::CTool;
use crate::tool::CToolSpec;
use crate::tools::git::run_git;

pub const CTOOL_GIT_DIFF_FILE_TOOL_NAME: &str = "ctool_git_diff_file";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolGitDiffFileInput {
    pub path: PathBuf,
    #[serde(default)]
    pub staged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolGitDiffFileOutput {
    pub current_dir: String,
    pub path: String,
    pub staged: bool,
    pub diff: String,
}

pub struct CToolGitDiffFile;

impl CTool for CToolGitDiffFile {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_GIT_DIFF_FILE_TOOL_NAME,
            description: "Read-only Git diff for one file inside current CToolScopeBase.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolGitDiffFileInput = serde_json::from_value(input)?;
        let output = git_diff_file(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

pub fn git_diff_file(
    ctx: &CToolContext,
    input: CToolGitDiffFileInput,
) -> CToolResult<CToolGitDiffFileOutput> {
    let checked_path = resolve_user_path(&ctx.scope_context, &input.path);
    if !can_read_path(&ctx.scope_context, &checked_path) {
        return Err(CToolError::OutOfScope {
            path: checked_path.display().to_string(),
            operation: "git diff",
        });
    }

    let current_dir = &ctx.scope_context.cool_workspace;
    let relative = checked_path.strip_prefix(current_dir).map_err(|_| {
        CToolError::InvalidInput("git diff file must be inside current CoolWorkspace".to_string())
    })?;
    let path_arg = relative.to_string_lossy().replace('\\', "/");
    let diff = if input.staged {
        run_git(current_dir, &["diff", "--cached", "--", &path_arg])?
    } else {
        run_git(current_dir, &["diff", "--", &path_arg])?
    };

    Ok(CToolGitDiffFileOutput {
        current_dir: current_dir.display().to_string(),
        path: checked_path.display().to_string(),
        staged: input.staged,
        diff,
    })
}
