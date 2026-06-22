use std::path::Path;

use pretty_assertions::assert_eq;

use crate::context::CToolContext;
use crate::error::CToolError;
use crate::scope::CToolScopeBase;
use crate::scope_config::empty_scope_config;
use crate::scope_context::CToolScopeContext;

use super::*;

#[test]
fn yaml_front_matter_requires_marker_line() {
    let ranges = markdown_protected_ranges("--- heading\nbody\n").unwrap();

    assert_eq!(ranges, Vec::<(usize, usize)>::new());
}

#[test]
fn protected_ranges_cover_markdown_regions() {
    let text = "---\ntitle: Demo\n---\n\nvisible `inline code` text\n<!-- comment -->\n```rust\ncode\n```\nvisible\n";
    let ranges = markdown_protected_ranges(text).unwrap();

    let inline_start = text.find("`inline code`").unwrap();
    let inline_end = inline_start + "`inline code`".len();
    let comment_start = text.find("<!-- comment -->").unwrap();
    let comment_end = comment_start + "<!-- comment -->".len();
    let fence_start = text.find("```rust").unwrap();
    let fence_end = text.find("visible\n").unwrap();

    assert!(range_is_protected(
        0,
        "---\ntitle: Demo\n---\n".len(),
        &ranges
    ));
    assert!(range_is_protected(inline_start, inline_end, &ranges));
    assert!(range_is_protected(comment_start, comment_end, &ranges));
    assert!(range_is_protected(fence_start, fence_end, &ranges));
}

#[test]
fn duplicate_targets_require_occurrence_choice() {
    let matches = find_target_matches("alpha beta alpha", "alpha");

    assert_eq!(matches, vec![(0, 5), (11, 16)]);
}

#[test]
fn annotation_direction_prefixes_use_arrows() {
    assert_eq!(
        annotation_direction_prefix(CToolMarkdownAnnotationDirection::Up),
        "↑"
    );
    assert_eq!(
        annotation_direction_prefix(CToolMarkdownAnnotationDirection::Down),
        "↓"
    );
}
#[test]
fn readonly_file_cannot_be_annotated_even_when_allow_readonly_is_true() {
    let root = unique_test_root("readonly_file_cannot_be_annotated");
    std::fs::create_dir_all(&root).unwrap();
    let root = std::fs::canonicalize(&root).unwrap();
    let path = root.join("note.md");
    std::fs::write(&path, "alpha beta").unwrap();

    let mut scope_context = test_scope_context(&root);
    scope_context.base_scope = CToolScopeBase::CoolWorkspace;
    scope_context.user_config.files.readonly = vec![path.clone()];
    let ctx = CToolContext::new(scope_context);

    let result = annotate_markdown(
        &ctx,
        CToolAnnotateMarkdownInput {
            path: path.clone(),
            target_text: "alpha".to_string(),
            annotation_kind: CToolMarkdownAnnotationKind::Normal,
            annotation_direction: None,
            occurrence: None,
            allow_readonly: true,
            dry_run: false,
        },
    );

    assert!(matches!(
        result,
        Err(CToolError::OutOfScope {
            operation: "write",
            ..
        })
    ));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "alpha beta");

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn readwrite_file_can_be_annotated() {
    let root = unique_test_root("readwrite_file_can_be_annotated");
    std::fs::create_dir_all(&root).unwrap();
    let root = std::fs::canonicalize(&root).unwrap();
    let path = root.join("note.md");
    std::fs::write(&path, "alpha beta").unwrap();

    let mut scope_context = test_scope_context(&root);
    scope_context.base_scope = CToolScopeBase::CoolWorkspace;
    scope_context.user_config.files.readwrite = vec![path.clone()];
    let ctx = CToolContext::new(scope_context);

    let output = annotate_markdown(
        &ctx,
        CToolAnnotateMarkdownInput {
            path: path.clone(),
            target_text: "alpha".to_string(),
            annotation_kind: CToolMarkdownAnnotationKind::Normal,
            annotation_direction: None,
            occurrence: None,
            allow_readonly: false,
            dry_run: false,
        },
    )
    .unwrap();

    assert!(output.success);
    assert_eq!(output.readonly_exception_used, false);
    assert!(std::fs::read_to_string(&path).unwrap().contains("<mark"));

    std::fs::remove_dir_all(&root).ok();
}

fn unique_test_root(name: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    std::env::temp_dir().join(format!(
        "ctool_annotate_markdown_tests_{name}_{}_{}",
        std::process::id(),
        nanos
    ))
}

fn test_scope_context(root: &Path) -> CToolScopeContext {
    CToolScopeContext {
        current_dir: root.to_path_buf(),
        character_root: root.to_path_buf(),
        cool_workspace: root.to_path_buf(),
        base_scope: CToolScopeBase::CoolWorkspace,
        user_config_path: root.join(".cool").join("scope.toml"),
        character_config_path: root.join(".cool").join("config.toml"),
        system_config_path: None,
        character_command_path: root.join(".cool").join("command.toml"),
        system_command_path: None,
        cool_system_dir: None,
        user_config: empty_scope_config(),
        system_config: empty_scope_config(),
    }
}
fn range_is_protected(start: usize, end: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|&(protected_start, protected_end)| start >= protected_start && end <= protected_end)
}
