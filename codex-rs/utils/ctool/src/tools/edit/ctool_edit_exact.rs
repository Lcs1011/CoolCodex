use std::path::Path;
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
use crate::tools::edit::ensure_editable_text_file_size;

pub const CTOOL_EDIT_REPLACE_EXACT_TOOL_NAME: &str = "ctool_edit_replace_exact";
pub const CTOOL_EDIT_INSERT_BEFORE_EXACT_TOOL_NAME: &str = "ctool_edit_insert_before_exact";
pub const CTOOL_EDIT_INSERT_AFTER_EXACT_TOOL_NAME: &str = "ctool_edit_insert_after_exact";
pub const CTOOL_EDIT_REMOVE_EXACT_TOOL_NAME: &str = "ctool_edit_remove_exact";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolEditReplaceExactInput {
    pub path: PathBuf,
    pub anchor: String,
    pub target: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolEditInsertBeforeExactInput {
    pub path: PathBuf,
    pub anchor: String,
    pub target: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolEditInsertAfterExactInput {
    pub path: PathBuf,
    pub anchor: String,
    pub target: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CToolEditRemoveExactInput {
    pub path: PathBuf,
    pub anchor: String,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CToolEditExactOutput {
    pub path: String,
    pub operation: String,
    pub matched_anchor: usize,
    pub matched_target: usize,
    pub byte_len_before: usize,
    pub byte_len_after: usize,
}

pub struct CToolEditReplaceExact;
pub struct CToolEditInsertBeforeExact;
pub struct CToolEditInsertAfterExact;
pub struct CToolEditRemoveExact;

impl CTool for CToolEditReplaceExact {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_EDIT_REPLACE_EXACT_TOOL_NAME,
            description: "Replace Target inside one exact Anchor in a UTF-8 file inside CToolScopeBase.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolEditReplaceExactInput = serde_json::from_value(input)?;
        let output = edit_replace_exact(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

impl CTool for CToolEditInsertBeforeExact {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_EDIT_INSERT_BEFORE_EXACT_TOOL_NAME,
            description: "Insert Content before Target inside one exact Anchor in a UTF-8 file inside CToolScopeBase.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolEditInsertBeforeExactInput = serde_json::from_value(input)?;
        let output = edit_insert_before_exact(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

impl CTool for CToolEditInsertAfterExact {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_EDIT_INSERT_AFTER_EXACT_TOOL_NAME,
            description: "Insert Content after Target inside one exact Anchor in a UTF-8 file inside CToolScopeBase.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolEditInsertAfterExactInput = serde_json::from_value(input)?;
        let output = edit_insert_after_exact(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

impl CTool for CToolEditRemoveExact {
    fn spec(&self) -> CToolSpec {
        CToolSpec {
            name: CTOOL_EDIT_REMOVE_EXACT_TOOL_NAME,
            description: "Remove Target inside one exact Anchor in a UTF-8 file inside CToolScopeBase.",
        }
    }

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value> {
        let input: CToolEditRemoveExactInput = serde_json::from_value(input)?;
        let output = edit_remove_exact(ctx, input)?;
        Ok(serde_json::to_value(output)?)
    }
}

pub fn edit_replace_exact(
    ctx: &CToolContext,
    input: CToolEditReplaceExactInput,
) -> CToolResult<CToolEditExactOutput> {
    let path = prepare_exact_edit_file(ctx, &input.path, "edit_replace_exact")?;
    let before = std::fs::read_to_string(&path)?;
    let after = apply_replace_exact_to_text(&before, &input.anchor, &input.target, &input.content)?;
    std::fs::write(&path, &after)?;

    exact_output(&path, "replace_exact", before.len(), after.len())
}

pub fn edit_insert_before_exact(
    ctx: &CToolContext,
    input: CToolEditInsertBeforeExactInput,
) -> CToolResult<CToolEditExactOutput> {
    let path = prepare_exact_edit_file(ctx, &input.path, "edit_insert_before_exact")?;
    let before = std::fs::read_to_string(&path)?;
    let after =
        apply_insert_before_exact_to_text(&before, &input.anchor, &input.target, &input.content)?;
    std::fs::write(&path, &after)?;

    exact_output(&path, "insert_before_exact", before.len(), after.len())
}

pub fn edit_insert_after_exact(
    ctx: &CToolContext,
    input: CToolEditInsertAfterExactInput,
) -> CToolResult<CToolEditExactOutput> {
    let path = prepare_exact_edit_file(ctx, &input.path, "edit_insert_after_exact")?;
    let before = std::fs::read_to_string(&path)?;
    let after =
        apply_insert_after_exact_to_text(&before, &input.anchor, &input.target, &input.content)?;
    std::fs::write(&path, &after)?;

    exact_output(&path, "insert_after_exact", before.len(), after.len())
}

pub fn edit_remove_exact(
    ctx: &CToolContext,
    input: CToolEditRemoveExactInput,
) -> CToolResult<CToolEditExactOutput> {
    let path = prepare_exact_edit_file(ctx, &input.path, "edit_remove_exact")?;
    let before = std::fs::read_to_string(&path)?;
    let after = apply_remove_exact_to_text(&before, &input.anchor, &input.target)?;
    std::fs::write(&path, &after)?;

    exact_output(&path, "remove_exact", before.len(), after.len())
}

pub fn apply_replace_exact_to_text(
    text: &str,
    anchor: &str,
    target: &str,
    content: &str,
) -> CToolResult<String> {
    let (_, _, target_start, target_end) = exact_anchor_target_ranges(text, anchor, target)?;

    let mut output = String::with_capacity(text.len() - target.len() + content.len());
    output.push_str(&text[..target_start]);
    output.push_str(content);
    output.push_str(&text[target_end..]);
    Ok(output)
}

pub fn apply_insert_before_exact_to_text(
    text: &str,
    anchor: &str,
    target: &str,
    content: &str,
) -> CToolResult<String> {
    ensure_non_empty_content(content)?;
    let (_, _, target_start, _) = exact_anchor_target_ranges(text, anchor, target)?;

    let mut output = String::with_capacity(text.len() + content.len());
    output.push_str(&text[..target_start]);
    output.push_str(content);
    output.push_str(&text[target_start..]);
    Ok(output)
}

pub fn apply_insert_after_exact_to_text(
    text: &str,
    anchor: &str,
    target: &str,
    content: &str,
) -> CToolResult<String> {
    ensure_non_empty_content(content)?;
    let (_, _, _, target_end) = exact_anchor_target_ranges(text, anchor, target)?;

    let mut output = String::with_capacity(text.len() + content.len());
    output.push_str(&text[..target_end]);
    output.push_str(content);
    output.push_str(&text[target_end..]);
    Ok(output)
}

pub fn apply_remove_exact_to_text(text: &str, anchor: &str, target: &str) -> CToolResult<String> {
    let (_, _, target_start, target_end) = exact_anchor_target_ranges(text, anchor, target)?;

    let mut output = String::with_capacity(text.len() - target.len());
    output.push_str(&text[..target_start]);
    output.push_str(&text[target_end..]);
    Ok(output)
}

fn prepare_exact_edit_file(
    ctx: &CToolContext,
    path: &Path,
    operation: &'static str,
) -> CToolResult<PathBuf> {
    gate::ensure_read_allowed(ctx, path)?;
    let path = gate::ensure_write_allowed(ctx, path)?;
    ensure_editable_text_file_size(&path, operation)?;
    Ok(path)
}

fn exact_output(
    path: &Path,
    operation: &str,
    byte_len_before: usize,
    byte_len_after: usize,
) -> CToolResult<CToolEditExactOutput> {
    Ok(CToolEditExactOutput {
        path: path.display().to_string(),
        operation: operation.to_string(),
        matched_anchor: 1,
        matched_target: 1,
        byte_len_before,
        byte_len_after,
    })
}

fn exact_anchor_target_ranges(
    text: &str,
    anchor: &str,
    target: &str,
) -> CToolResult<(usize, usize, usize, usize)> {
    let (anchor_start, anchor_end) = unique_match_range(text, anchor, "anchor", "")?;
    let anchor_text = &text[anchor_start..anchor_end];
    let (target_relative_start, target_relative_end) =
        unique_match_range(anchor_text, target, "target", " inside anchor")?;

    Ok((
        anchor_start,
        anchor_end,
        anchor_start + target_relative_start,
        anchor_start + target_relative_end,
    ))
}

fn unique_match_range(
    text: &str,
    needle: &str,
    label: &str,
    scope_note: &str,
) -> CToolResult<(usize, usize)> {
    if needle.is_empty() {
        return Err(CToolError::InvalidInput(format!(
            "{label} must not be empty"
        )));
    }

    let count = text.matches(needle).count();
    if count == 0 {
        return Err(CToolError::InvalidInput(format!(
            "{label} was not found{scope_note}"
        )));
    }

    if count > 1 {
        return Err(CToolError::InvalidInput(format!(
            "{label} matched {count} times{scope_note}; exact edit requires exactly one {label} match{scope_note}"
        )));
    }

    let start = text
        .find(needle)
        .expect("text.matches counted exactly one match");
    Ok((start, start + needle.len()))
}

fn ensure_non_empty_content(content: &str) -> CToolResult<()> {
    if content.is_empty() {
        return Err(CToolError::InvalidInput(
            "content must not be empty".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_exact_replaces_target_inside_unique_anchor() {
        let text = "fn a() {\n    let value = 1;\n}\n\nfn b() {\n    let value = 1;\n}\n";
        let anchor = "fn b() {\n    let value = 1;\n}\n";
        let after = apply_replace_exact_to_text(
            text,
            anchor,
            "    let value = 1;\n",
            "    let value = 2;\n",
        )
        .unwrap();

        assert_eq!(
            after,
            "fn a() {\n    let value = 1;\n}\n\nfn b() {\n    let value = 2;\n}\n"
        );
    }

    #[test]
    fn insert_before_exact_inserts_before_target_inside_anchor() {
        let text = "fn demo() {\n    println!(\"end\");\n}\n";
        let anchor = "fn demo() {\n    println!(\"end\");\n}\n";
        let after = apply_insert_before_exact_to_text(
            text,
            anchor,
            "    println!(\"end\");\n",
            "    println!(\"start\");\n",
        )
        .unwrap();

        assert_eq!(
            after,
            "fn demo() {\n    println!(\"start\");\n    println!(\"end\");\n}\n"
        );
    }

    #[test]
    fn insert_after_exact_inserts_after_target_inside_anchor() {
        let text = "fn demo() {\n    println!(\"start\");\n}\n";
        let anchor = "fn demo() {\n    println!(\"start\");\n}\n";
        let after = apply_insert_after_exact_to_text(
            text,
            anchor,
            "    println!(\"start\");\n",
            "    println!(\"end\");\n",
        )
        .unwrap();

        assert_eq!(
            after,
            "fn demo() {\n    println!(\"start\");\n    println!(\"end\");\n}\n"
        );
    }

    #[test]
    fn remove_exact_removes_target_inside_anchor() {
        let text = "fn demo() {\n    println!(\"start\");\n    println!(\"debug\");\n    println!(\"end\");\n}\n";
        let anchor = "fn demo() {\n    println!(\"start\");\n    println!(\"debug\");\n    println!(\"end\");\n}\n";
        let after = apply_remove_exact_to_text(text, anchor, "    println!(\"debug\");\n").unwrap();

        assert_eq!(
            after,
            "fn demo() {\n    println!(\"start\");\n    println!(\"end\");\n}\n"
        );
    }

    #[test]
    fn exact_edit_rejects_missing_anchor() {
        let error = apply_remove_exact_to_text("alpha", "missing", "alpha").unwrap_err();

        assert!(
            matches!(error, CToolError::InvalidInput(message) if message.contains("anchor was not found"))
        );
    }

    #[test]
    fn exact_edit_rejects_duplicate_anchor() {
        let error = apply_remove_exact_to_text("alpha\nalpha\n", "alpha", "alpha").unwrap_err();

        assert!(
            matches!(error, CToolError::InvalidInput(message) if message.contains("anchor matched 2 times"))
        );
    }

    #[test]
    fn exact_edit_rejects_missing_target_inside_anchor() {
        let error = apply_remove_exact_to_text("alpha beta", "alpha beta", "missing").unwrap_err();

        assert!(
            matches!(error, CToolError::InvalidInput(message) if message.contains("target was not found inside anchor"))
        );
    }

    #[test]
    fn exact_edit_rejects_duplicate_target_inside_anchor() {
        let error = apply_remove_exact_to_text("alpha alpha", "alpha alpha", "alpha").unwrap_err();

        assert!(
            matches!(error, CToolError::InvalidInput(message) if message.contains("target matched 2 times inside anchor"))
        );
    }
}
