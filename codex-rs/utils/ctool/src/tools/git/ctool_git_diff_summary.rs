use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::context::CToolContext;
use crate::error::CToolResult;
use crate::scope_context::can_read_path;
use crate::scope_context::resolve_user_path;
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
pub struct CToolGitDiffSummaryVisibleFile {
    pub path: String,
    pub status: String,
    pub diff_stat: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolGitDiffSummaryOutput {
    pub current_dir: String,
    pub visible_files: Vec<CToolGitDiffSummaryVisibleFile>,
    pub hidden_or_out_of_scope_count: usize,
}

pub struct CToolGitDiffSummary;

impl CTool for CToolGitDiffSummary {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_GIT_DIFF_SUMMARY_TOOL_NAME,
            description: "Read-only Git status summary filtered through current CToolScopeBase.",
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
    let status_raw = run_git(current_dir, &["status", "--porcelain=v1", "-z"])?;
    let changed_files = parse_git_status_z(&status_raw);

    let mut visible_files = Vec::new();
    let mut hidden_or_out_of_scope_count = 0usize;

    for changed_file in changed_files {
        let resolved = resolve_user_path(&ctx.scope_context, &changed_file.path);
        let related_resolved = changed_file
            .related_path
            .as_ref()
            .map(|path| resolve_user_path(&ctx.scope_context, path));
        if !is_visible_git_path(ctx, current_dir, &resolved)
            || related_resolved
                .as_deref()
                .map_or(false, |path| !is_visible_git_path(ctx, current_dir, path))
        {
            hidden_or_out_of_scope_count += 1;
            continue;
        }

        let Some(relative) = relative_git_path(current_dir, &resolved) else {
            hidden_or_out_of_scope_count += 1;
            continue;
        };
        let diff_stat = if input.include_stat && !is_rename_or_copy_status(&changed_file.status) {
            visible_diff_stat(current_dir, &relative)?
        } else {
            None
        };

        visible_files.push(CToolGitDiffSummaryVisibleFile {
            path: relative,
            status: changed_file.status,
            diff_stat,
        });
    }

    Ok(CToolGitDiffSummaryOutput {
        current_dir: current_dir.display().to_string(),
        visible_files,
        hidden_or_out_of_scope_count,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GitChangedFile {
    status: String,
    path: PathBuf,
    related_path: Option<PathBuf>,
}

fn parse_git_status_z(status_raw: &str) -> Vec<GitChangedFile> {
    let entries = status_raw
        .split('\0')
        .filter(|entry| !entry.is_empty())
        .collect::<Vec<_>>();
    let mut output = Vec::new();
    let mut index = 0usize;

    while index < entries.len() {
        let entry = entries[index];
        index += 1;
        if entry.len() < 4 {
            continue;
        }

        let status = entry[..2].to_string();
        let path = PathBuf::from(&entry[3..]);
        let related_path = if is_rename_or_copy_status(&status) && index < entries.len() {
            let related = PathBuf::from(entries[index]);
            index += 1;
            Some(related)
        } else {
            None
        };

        output.push(GitChangedFile {
            status,
            path,
            related_path,
        });
    }

    output
}

fn is_rename_or_copy_status(status: &str) -> bool {
    status
        .as_bytes()
        .first()
        .map_or(false, |byte| matches!(*byte, b'R' | b'C'))
        || status
            .as_bytes()
            .get(1)
            .map_or(false, |byte| matches!(*byte, b'R' | b'C'))
}

fn visible_diff_stat(current_dir: &Path, relative_path: &str) -> CToolResult<Option<String>> {
    let unstaged = run_git(current_dir, &["diff", "--stat", "--", relative_path])?;
    let staged = run_git(
        current_dir,
        &["diff", "--cached", "--stat", "--", relative_path],
    )?;
    let mut parts = Vec::new();
    if !unstaged.trim().is_empty() {
        parts.push(unstaged.trim().to_string());
    }
    if !staged.trim().is_empty() {
        parts.push(staged.trim().to_string());
    }

    if parts.is_empty() {
        Ok(None)
    } else {
        Ok(Some(parts.join("\n")))
    }
}

fn is_visible_git_path(ctx: &CToolContext, current_dir: &Path, path: &Path) -> bool {
    path_is_inside(current_dir, path) && can_read_path(&ctx.scope_context, path)
}

fn relative_git_path(current_dir: &Path, path: &Path) -> Option<String> {
    Some(
        path.strip_prefix(current_dir)
            .ok()?
            .to_string_lossy()
            .replace('\\', "/"),
    )
}

fn path_is_inside(root: &Path, path: &Path) -> bool {
    path == root || path.starts_with(root)
}
