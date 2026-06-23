use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::context::CToolContext;
use crate::error::CToolResult;
use crate::tool::CTool;
use crate::tool::CToolSpec;
use crate::tools::git::run_git;

pub const CTOOL_GIT_DIFF_SUMMARY_TOOL_NAME: &str = "ctool_git_diff_summary";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolGitDiffSummaryInput {
    #[serde(default)]
    pub include_stat: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolGitDiffSummaryOutput {
    pub current_dir: String,
    pub status_short: String,
    pub diff_stat: Option<String>,
}

pub struct CToolGitDiffSummary;

impl CTool for CToolGitDiffSummary {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_GIT_DIFF_SUMMARY_TOOL_NAME,
            description: "Read-only Git status and optional diff stat for the current CoolWorkspace.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolGitDiffSummaryInput = serde_json::from_value(input)?;
        let output = git_diff_summary(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

pub fn git_diff_summary(
    ctx: &CToolContext,
    input: CToolGitDiffSummaryInput,
) -> CToolResult<CToolGitDiffSummaryOutput> {
    let current_dir = &ctx.scope_context.cool_workspace;
    let status_short = run_git(current_dir, &["status", "--short"])?;
    let diff_stat = if input.include_stat {
        Some(run_git(current_dir, &["diff", "--stat"])?)
    } else {
        None
    };

    Ok(CToolGitDiffSummaryOutput {
        current_dir: current_dir.display().to_string(),
        status_short,
        diff_stat,
    })
}
