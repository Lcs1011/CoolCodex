use serde_json::Map;
use serde_json::Value;
use serde_json::json;

#[cfg(test)]
use crate::tool::CToolSpec;

pub fn ctool_input_schema(name: &str) -> Value {
    match name {
        "ctool_command_request" => object_schema(
            &["commands"],
            vec![
                (
                    "commands",
                    array_schema(
                        string_schema(
                            "Command line to preview and optionally execute. Each command is evaluated by command.toml and hard-risk rules.",
                        ),
                        "Ordered command list. Each command is evaluated independently against sandbox restrictions, command.toml policy, and hard-risk rules.",
                    ),
                ),
                (
                    "ai_risk_upgrade",
                    enum_schema(
                        &["green", "yellow", "red", "blocked"],
                        "Optional AI-side risk upgrade. Cannot lower system risk; it can only keep or raise the final risk.",
                    ),
                ),
                (
                    "reason",
                    string_schema("Optional request reason shown in audit output."),
                ),
                (
                    "yellow_confirmation",
                    string_schema("Required only when final risk is YELLOW."),
                ),
                (
                    "red_first_confirmation",
                    string_schema("Required first confirmation only when final risk is RED."),
                ),
                (
                    "red_second_confirmation",
                    string_schema("Required second confirmation only when final risk is RED."),
                ),
            ],
        ),
        "ctool_tavily_search_request" => object_schema(
            &["action", "query"],
            vec![
                (
                    "action",
                    enum_schema(&["search"], "V1 supports only public text search."),
                ),
                (
                    "query",
                    string_schema(
                        "Public web search query. Do not include local file contents or secrets.",
                    ),
                ),
                (
                    "file_name_hint",
                    string_schema("Optional safe filename hint."),
                ),
                (
                    "yellow_confirmation",
                    string_schema("Reserved for future non-green Tavily actions."),
                ),
                (
                    "red_first_confirmation",
                    string_schema("Reserved for future red Tavily actions."),
                ),
                (
                    "red_second_confirmation",
                    string_schema("Reserved for future red Tavily actions."),
                ),
            ],
        ),
        "ctool_list_directory" => object_schema(
            &["path"],
            vec![
                (
                    "path",
                    path_schema("Directory path inside current CToolScopeBase."),
                ),
                ("max_depth", integer_schema("Maximum recursion depth.")),
                ("max_entries", integer_schema("Maximum returned entries.")),
                (
                    "include_hidden",
                    boolean_schema("Whether to include hidden files and directories."),
                ),
            ],
        ),
        "ctool_rg_search" => {
            search_input_schema("query", "Literal text query searched across UTF-8 files.")
        }
        "ctool_rg_search_context" => object_schema(
            &["path", "query"],
            vec![
                (
                    "path",
                    path_schema("Directory or file path inside current CToolScopeBase."),
                ),
                (
                    "query",
                    string_schema("Literal text query searched across UTF-8 files."),
                ),
                (
                    "before",
                    integer_schema("Number of context lines before each match."),
                ),
                (
                    "after",
                    integer_schema("Number of context lines after each match."),
                ),
                (
                    "case_sensitive",
                    boolean_schema("Whether matching is case-sensitive."),
                ),
                (
                    "max_depth",
                    integer_schema("Maximum directory recursion depth."),
                ),
                ("max_results", integer_schema("Maximum returned matches.")),
                (
                    "include_hidden",
                    boolean_schema("Whether to search hidden files and directories."),
                ),
            ],
        ),
        "ctool_regex_search" => {
            search_input_schema("pattern", "Rust regex pattern searched across UTF-8 files.")
        }
        "ctool_count_matches" => object_schema(
            &["path", "query"],
            vec![
                (
                    "path",
                    path_schema("Directory or file path inside current CToolScopeBase."),
                ),
                (
                    "query",
                    string_schema("Literal text or regex pattern to count."),
                ),
                (
                    "is_regex",
                    boolean_schema("Whether query is interpreted as a Rust regex pattern."),
                ),
                (
                    "case_sensitive",
                    boolean_schema("Whether matching is case-sensitive."),
                ),
                (
                    "max_depth",
                    integer_schema("Maximum directory recursion depth."),
                ),
                (
                    "include_hidden",
                    boolean_schema("Whether to search hidden files and directories."),
                ),
            ],
        ),
        "ctool_extract_lines_matching" => object_schema(
            &["path", "query"],
            vec![
                (
                    "path",
                    path_schema("Directory or file path inside current CToolScopeBase."),
                ),
                (
                    "query",
                    string_schema("Literal text or regex pattern used to extract matching lines."),
                ),
                (
                    "is_regex",
                    boolean_schema("Whether query is interpreted as a Rust regex pattern."),
                ),
                (
                    "case_sensitive",
                    boolean_schema("Whether matching is case-sensitive."),
                ),
                (
                    "unique",
                    boolean_schema("Whether duplicate extracted lines are removed."),
                ),
                (
                    "sort",
                    boolean_schema("Whether extracted lines are sorted."),
                ),
                (
                    "max_depth",
                    integer_schema("Maximum directory recursion depth."),
                ),
                ("max_results", integer_schema("Maximum returned lines.")),
                (
                    "include_hidden",
                    boolean_schema("Whether to search hidden files and directories."),
                ),
            ],
        ),
        "ctool_read_file" | "ctool_outline_file" | "ctool_tail_file" => object_schema(
            &["path"],
            vec![
                (
                    "path",
                    path_schema("UTF-8 text/source file path inside current CToolScopeBase."),
                ),
                (
                    "max_bytes",
                    integer_schema("Maximum bytes to read where supported."),
                ),
                (
                    "line_count",
                    integer_schema("Number of trailing lines for tail reads."),
                ),
            ],
        ),
        "ctool_read_code_range" => range_input_schema(),
        "ctool_read_many_ranges" => object_schema(
            &["path", "ranges"],
            vec![
                (
                    "path",
                    path_schema("UTF-8 text file path inside current CToolScopeBase."),
                ),
                (
                    "ranges",
                    array_schema(
                        object_open_schema("Range object with start_line and end_line."),
                        "Inclusive 1-based ranges.",
                    ),
                ),
            ],
        ),
        "ctool_read_symbol" => object_schema(
            &["path", "symbol"],
            vec![
                (
                    "path",
                    path_schema("UTF-8 source file path inside current CToolScopeBase."),
                ),
                ("symbol", string_schema("Symbol name to read.")),
                (
                    "kind",
                    string_schema("Optional kind filter: struct, enum, impl, fn, mod, or test."),
                ),
            ],
        ),
        "ctool_edit_replace" => object_schema(
            &["path", "old_string", "new_string"],
            vec![
                (
                    "path",
                    path_schema("Editable UTF-8 text file inside current CToolScopeBase."),
                ),
                (
                    "old_string",
                    string_schema("Exact old text to replace. Must match exactly once."),
                ),
                ("new_string", string_schema("Replacement text.")),
            ],
        ),
        "ctool_edit_insert" => object_schema(
            &["path", "insert_after_line", "content"],
            vec![
                (
                    "path",
                    path_schema("Editable UTF-8 text file inside current CToolScopeBase."),
                ),
                (
                    "insert_after_line",
                    integer_schema("0 inserts at file beginning; N inserts after line N."),
                ),
                ("content", string_schema("Text to insert.")),
            ],
        ),
        "ctool_preview_diff" | "ctool_edit_preview" => object_schema(
            &["path", "operations"],
            vec![
                (
                    "path",
                    path_schema("Editable UTF-8 file inside current CToolScopeBase."),
                ),
                (
                    "operations",
                    array_schema(
                        object_open_schema("Preview operation object."),
                        "Ordered preview operations. Nothing is written.",
                    ),
                ),
            ],
        ),
        "ctool_edit_batch" => object_schema(
            &["operations"],
            vec![(
                "operations",
                array_schema(
                    object_open_schema("One edit operation."),
                    "Ordered edit operations.",
                ),
            )],
        ),
        "ctool_edit_replace_exact" => {
            exact_edit_schema("Replace target text inside a unique anchor.")
        }
        "ctool_edit_insert_before_exact" => {
            exact_edit_schema("Insert content before target text inside a unique anchor.")
        }
        "ctool_edit_insert_after_exact" => {
            exact_edit_schema("Insert content after target text inside a unique anchor.")
        }
        "ctool_edit_remove_exact" => object_schema(
            &["path", "anchor", "target"],
            vec![
                (
                    "path",
                    path_schema("Editable UTF-8 text file inside current CToolScopeBase."),
                ),
                (
                    "anchor",
                    string_schema("Exact anchor text. Must match exactly once in the file."),
                ),
                ("target", string_schema("Exact target text inside anchor.")),
            ],
        ),
        "ctool_create_file" => object_schema(
            &["path", "content"],
            vec![
                (
                    "path",
                    path_schema("New file path inside current CToolScopeBase."),
                ),
                ("content", string_schema("UTF-8 file content to write.")),
                (
                    "overwrite",
                    boolean_schema("Whether existing file may be overwritten."),
                ),
            ],
        ),
        "ctool_delete_file" => object_schema(
            &["path"],
            vec![
                (
                    "path",
                    path_schema("File path inside current CToolScopeBase to delete."),
                ),
                (
                    "expected_content",
                    string_schema("Optional exact file content guard."),
                ),
            ],
        ),
        "ctool_move_file" => object_schema(
            &["from", "to"],
            vec![
                (
                    "from",
                    path_schema("Source file path inside current CToolScopeBase."),
                ),
                (
                    "to",
                    path_schema("Destination file path inside current CToolScopeBase."),
                ),
                (
                    "overwrite",
                    boolean_schema("Whether an existing destination file may be overwritten."),
                ),
            ],
        ),
        "ctool_create_directory" | "ctool_delete_directory" => object_schema(
            &["path"],
            vec![(
                "path",
                path_schema("Directory path inside current CToolScopeBase."),
            )],
        ),
        "ctool_move_directory" => object_schema(
            &["from", "to"],
            vec![
                (
                    "from",
                    path_schema("Source directory path inside current CToolScopeBase."),
                ),
                (
                    "to",
                    path_schema("Destination directory path inside current CToolScopeBase."),
                ),
            ],
        ),
        "ctool_git_diff_summary" => object_schema(
            &[],
            vec![(
                "include_stat",
                boolean_schema("Whether to include git diff --stat output."),
            )],
        ),
        "ctool_git_diff_file" => object_schema(
            &["path"],
            vec![
                (
                    "path",
                    path_schema("File path inside current CToolScopeBase to diff."),
                ),
                (
                    "staged",
                    boolean_schema("Whether to show staged diff with git diff --cached."),
                ),
            ],
        ),
        "ctool_annotate_markdown" => object_schema(
            &["path", "target_text"],
            vec![
                (
                    "path",
                    path_schema("Markdown file path inside current CToolScopeBase."),
                ),
                (
                    "target_text",
                    string_schema("Exact Markdown text to wrap with a safe annotation."),
                ),
                (
                    "annotation_kind",
                    enum_schema(&["normal", "important"], "Annotation color/intensity."),
                ),
                (
                    "annotation_direction",
                    enum_schema(&["up", "down"], "Optional arrow direction."),
                ),
                (
                    "occurrence",
                    integer_schema("1-based occurrence when target appears more than once."),
                ),
                (
                    "allow_readonly",
                    boolean_schema("Reserved compatibility flag."),
                ),
                (
                    "dry_run",
                    boolean_schema("Preview annotation without writing the file."),
                ),
            ],
        ),
        _ => object_open_schema("Unknown CTool input schema."),
    }
}

pub fn ctool_output_schema(name: &str) -> Value {
    match name {
        "ctool_command_request" => object_schema(
            &[],
            vec![
                (
                    "will_execute",
                    boolean_schema("Whether the request will execute."),
                ),
                ("executed", boolean_schema("Whether commands executed.")),
                (
                    "blocked",
                    boolean_schema("Whether the request was blocked."),
                ),
                (
                    "rejected",
                    boolean_schema("Whether the user rejected confirmation."),
                ),
                (
                    "all_success",
                    boolean_schema("Whether all executed commands succeeded."),
                ),
                (
                    "result_file",
                    string_schema("Optional Markdown result file path."),
                ),
                ("log_file", string_schema("Optional audit log file path.")),
                (
                    "current_dir",
                    string_schema("Command execution current directory."),
                ),
                (
                    "command_count",
                    integer_schema("Number of requested commands."),
                ),
                (
                    "system_risk",
                    enum_schema(
                        &["GREEN", "YELLOW", "RED", "BLOCKED"],
                        "Risk before AI-side upgrade.",
                    ),
                ),
                (
                    "ai_risk_upgrade",
                    enum_schema(
                        &["GREEN", "YELLOW", "RED", "BLOCKED"],
                        "Optional AI-side risk upgrade.",
                    ),
                ),
                (
                    "final_risk",
                    enum_schema(&["GREEN", "YELLOW", "RED", "BLOCKED"], "Final risk."),
                ),
                ("approval_required", string_schema("Approval mode.")),
                ("request_reason", string_schema("Optional request reason.")),
                (
                    "user_feedback",
                    string_schema("Optional rejection feedback."),
                ),
                (
                    "commands",
                    array_schema(
                        object_open_schema("Command preview item."),
                        "Per-command preview list.",
                    ),
                ),
                (
                    "display_text",
                    string_schema("Human-readable preview text."),
                ),
                ("banner", string_schema("Human-readable risk banner.")),
                ("note", string_schema("Human-readable note.")),
            ],
        ),
        "ctool_tavily_search_request" => object_schema(
            &[],
            vec![
                (
                    "will_execute",
                    boolean_schema("Whether the request will execute."),
                ),
                ("executed", boolean_schema("Whether Tavily was called.")),
                (
                    "blocked",
                    boolean_schema("Whether the request was blocked."),
                ),
                (
                    "rejected",
                    boolean_schema("Whether the user rejected confirmation."),
                ),
                ("current_dir", string_schema("Current CoolWorkspace path.")),
                ("action", enum_schema(&["search"], "V1 action label.")),
                (
                    "final_risk",
                    enum_schema(&["GREEN", "YELLOW", "RED", "BLOCKED"], "Final risk label."),
                ),
                ("output_file", string_schema("Cached Markdown result path.")),
                ("log_file", string_schema("Audit log path.")),
                ("summary", string_schema("Short summary.")),
                (
                    "suggested_next_step",
                    string_schema("Suggested next action."),
                ),
                (
                    "user_feedback",
                    string_schema("Optional rejection feedback."),
                ),
                ("note", string_schema("Human-readable note.")),
            ],
        ),
        "ctool_list_directory" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved directory root path.")),
                (
                    "total_returned",
                    integer_schema("Number of returned entries."),
                ),
                (
                    "truncated",
                    boolean_schema("Whether results were truncated."),
                ),
                (
                    "items",
                    array_schema(object_open_schema("Directory item."), "Directory entries."),
                ),
            ],
        ),
        "ctool_rg_search" | "ctool_regex_search" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved search root path.")),
                ("query", string_schema("Search query.")),
                ("pattern", string_schema("Search pattern.")),
                (
                    "total_returned",
                    integer_schema("Number of returned matches."),
                ),
                (
                    "truncated",
                    boolean_schema("Whether results were truncated."),
                ),
                (
                    "matches",
                    array_schema(object_open_schema("Search match."), "Matches."),
                ),
            ],
        ),
        "ctool_rg_search_context" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved search root path.")),
                ("query", string_schema("Search query.")),
                (
                    "total_returned",
                    integer_schema("Number of returned matches."),
                ),
                (
                    "truncated",
                    boolean_schema("Whether results were truncated."),
                ),
                (
                    "matches",
                    array_schema(
                        object_open_schema("Context match."),
                        "Matches with context.",
                    ),
                ),
            ],
        ),
        "ctool_count_matches" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved search root path.")),
                ("query", string_schema("Query used for counting.")),
                ("is_regex", boolean_schema("Whether regex was used.")),
                (
                    "matching_file_count",
                    integer_schema("Number of files with matches."),
                ),
                (
                    "line_match_count",
                    integer_schema("Number of matching lines."),
                ),
            ],
        ),
        "ctool_extract_lines_matching" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved search root path.")),
                ("query", string_schema("Query used for extraction.")),
                ("is_regex", boolean_schema("Whether regex was used.")),
                (
                    "total_returned",
                    integer_schema("Number of returned lines."),
                ),
                (
                    "truncated",
                    boolean_schema("Whether results were truncated."),
                ),
                (
                    "lines",
                    array_schema(object_open_schema("Extracted line."), "Extracted lines."),
                ),
            ],
        ),
        "ctool_read_file" => object_schema(
            &[],
            vec![
                ("path", string_schema("Resolved file path.")),
                ("byte_len", integer_schema("File byte length.")),
                (
                    "truncated",
                    boolean_schema("Whether content was truncated."),
                ),
                ("content", string_schema("UTF-8 file content.")),
            ],
        ),
        "ctool_outline_file" => object_schema(
            &[],
            vec![
                ("path", string_schema("Resolved file path.")),
                ("total_lines", integer_schema("Total line count.")),
                (
                    "symbol_count",
                    integer_schema("Number of returned symbols."),
                ),
                (
                    "truncated",
                    boolean_schema("Whether symbols were truncated."),
                ),
                (
                    "symbols",
                    array_schema(
                        object_open_schema("Symbol item."),
                        "Detected source symbols.",
                    ),
                ),
            ],
        ),
        "ctool_read_code_range" => object_schema(
            &[],
            vec![
                ("path", string_schema("Resolved file path.")),
                (
                    "start_line",
                    integer_schema("1-based inclusive start line."),
                ),
                ("end_line", integer_schema("1-based inclusive end line.")),
                ("content", string_schema("UTF-8 code range content.")),
            ],
        ),
        "ctool_read_many_ranges" => object_schema(
            &[],
            vec![
                ("path", string_schema("Resolved file path.")),
                ("total_lines", integer_schema("Total line count.")),
                ("range_count", integer_schema("Number of returned ranges.")),
                (
                    "ranges",
                    array_schema(object_open_schema("Range output."), "Returned ranges."),
                ),
            ],
        ),
        "ctool_read_symbol" => object_schema(
            &[],
            vec![
                ("path", string_schema("Resolved file path.")),
                ("symbol", string_schema("Matched symbol name.")),
                ("kind", string_schema("Matched symbol kind.")),
                (
                    "start_line",
                    integer_schema("1-based inclusive start line."),
                ),
                ("end_line", integer_schema("1-based inclusive end line.")),
                ("total_lines", integer_schema("Total file line count.")),
                ("signature", string_schema("Matched signature.")),
                ("content", string_schema("Matched symbol source content.")),
            ],
        ),
        "ctool_tail_file" => object_schema(
            &[],
            vec![
                ("path", string_schema("Resolved file path.")),
                ("line_count", integer_schema("Number of returned lines.")),
                ("content", string_schema("Trailing UTF-8 file content.")),
            ],
        ),
        "ctool_edit_replace" => object_schema(
            &[],
            vec![
                ("path", string_schema("Edited file path.")),
                ("replaced", integer_schema("Number of replacements.")),
                (
                    "byte_len_before",
                    integer_schema("File byte length before edit."),
                ),
                (
                    "byte_len_after",
                    integer_schema("File byte length after edit."),
                ),
            ],
        ),
        "ctool_edit_insert" => object_schema(
            &[],
            vec![
                ("path", string_schema("Edited file path.")),
                (
                    "inserted_after_line",
                    integer_schema("Line after which content was inserted."),
                ),
                (
                    "byte_len_before",
                    integer_schema("File byte length before edit."),
                ),
                (
                    "byte_len_after",
                    integer_schema("File byte length after edit."),
                ),
            ],
        ),
        "ctool_preview_diff" | "ctool_edit_preview" => object_schema(
            &[],
            vec![
                ("path", string_schema("Target file path.")),
                (
                    "operation_count",
                    integer_schema("Number of preview operations."),
                ),
                (
                    "changed",
                    boolean_schema("Whether operation would change the file."),
                ),
                ("diff", string_schema("Simple unified diff preview.")),
            ],
        ),
        "ctool_edit_batch" => object_schema(
            &[],
            vec![
                (
                    "operation_count",
                    integer_schema("Number of applied edit operations."),
                ),
                (
                    "files_touched",
                    array_schema(
                        string_schema("Edited file path."),
                        "Files written by the batch.",
                    ),
                ),
            ],
        ),
        "ctool_edit_replace_exact"
        | "ctool_edit_insert_before_exact"
        | "ctool_edit_insert_after_exact"
        | "ctool_edit_remove_exact" => object_schema(
            &[],
            vec![
                ("path", string_schema("Edited file path.")),
                ("operation", string_schema("Exact edit operation label.")),
                (
                    "matched_anchor",
                    integer_schema("Number of matched anchors."),
                ),
                (
                    "matched_target",
                    integer_schema("Number of matched targets."),
                ),
                (
                    "byte_len_before",
                    integer_schema("File byte length before edit."),
                ),
                (
                    "byte_len_after",
                    integer_schema("File byte length after edit."),
                ),
            ],
        ),
        "ctool_annotate_markdown" => object_schema(
            &[],
            vec![
                ("success", boolean_schema("Whether annotation succeeded.")),
                (
                    "dry_run",
                    boolean_schema("Whether file was left unchanged."),
                ),
                (
                    "readonly_exception_used",
                    boolean_schema("Whether readonly exception was used."),
                ),
                (
                    "annotation_kind",
                    enum_schema(&["normal", "important"], "Annotation kind used."),
                ),
                (
                    "annotation_direction",
                    enum_schema(&["up", "down"], "Optional arrow direction used."),
                ),
                ("path", string_schema("Annotated Markdown file path.")),
                ("line_number", integer_schema("1-based line number.")),
                ("occurrence", integer_schema("1-based selected occurrence.")),
                (
                    "before_preview",
                    string_schema("Preview before annotation."),
                ),
                ("after_preview", string_schema("Preview after annotation.")),
                ("note", string_schema("Human-readable result note.")),
            ],
        ),
        "ctool_create_file" => object_schema(
            &[],
            vec![
                ("path", string_schema("Created file path.")),
                ("byte_len", integer_schema("Written byte length.")),
                (
                    "overwritten",
                    boolean_schema("Whether an existing file was overwritten."),
                ),
            ],
        ),
        "ctool_delete_file" => object_schema(
            &[],
            vec![
                ("path", string_schema("Deleted file path.")),
                (
                    "byte_len_before",
                    integer_schema("File byte length before deletion."),
                ),
                ("deleted", boolean_schema("Whether file was deleted.")),
            ],
        ),
        "ctool_move_file" => object_schema(
            &[],
            vec![
                ("from", string_schema("Resolved source file path.")),
                ("to", string_schema("Resolved destination file path.")),
                (
                    "overwritten",
                    boolean_schema("Whether destination was overwritten."),
                ),
                ("moved", boolean_schema("Whether file was moved.")),
            ],
        ),
        "ctool_create_directory" => object_schema(
            &[],
            vec![
                ("path", string_schema("Created directory path.")),
                (
                    "created",
                    boolean_schema("Whether directory was newly created."),
                ),
            ],
        ),
        "ctool_delete_directory" => object_schema(
            &[],
            vec![
                ("path", string_schema("Deleted directory path.")),
                ("deleted", boolean_schema("Whether directory was deleted.")),
            ],
        ),
        "ctool_move_directory" => object_schema(
            &[],
            vec![
                ("from", string_schema("Resolved source directory path.")),
                ("to", string_schema("Resolved destination directory path.")),
                ("moved", boolean_schema("Whether directory was moved.")),
            ],
        ),
        "ctool_git_diff_summary" => object_schema(
            &[],
            vec![
                ("current_dir", string_schema("Git working directory.")),
                (
                    "visible_files",
                    array_schema(
                        object_open_schema(
                            "Scope-visible changed file with status and optional per-file diff stat.",
                        ),
                        "Changed files that are readable under current CToolScopeBase.",
                    ),
                ),
                (
                    "hidden_or_out_of_scope_count",
                    integer_schema("Number of changed files hidden by scope filtering."),
                ),
            ],
        ),
        "ctool_git_diff_file" => object_schema(
            &[],
            vec![
                ("current_dir", string_schema("Git working directory.")),
                ("path", string_schema("Resolved file path.")),
                (
                    "staged",
                    boolean_schema("Whether staged diff was requested."),
                ),
                ("diff", string_schema("Git diff output for the file.")),
            ],
        ),
        _ => object_open_schema("Unknown CTool output schema."),
    }
}

fn search_input_schema(query_field: &str, query_description: &str) -> Value {
    object_schema(
        &["path", query_field],
        vec![
            (
                "path",
                path_schema("Directory or file path inside current CToolScopeBase."),
            ),
            (query_field, string_schema(query_description)),
            (
                "case_sensitive",
                boolean_schema("Whether matching is case-sensitive."),
            ),
            (
                "max_depth",
                integer_schema("Maximum directory recursion depth."),
            ),
            ("max_results", integer_schema("Maximum returned matches.")),
            (
                "include_hidden",
                boolean_schema("Whether to search hidden files and directories."),
            ),
        ],
    )
}

fn range_input_schema() -> Value {
    object_schema(
        &["path", "start_line", "end_line"],
        vec![
            (
                "path",
                path_schema("UTF-8 text file path inside current CToolScopeBase."),
            ),
            (
                "start_line",
                integer_schema("1-based inclusive start line."),
            ),
            ("end_line", integer_schema("1-based inclusive end line.")),
        ],
    )
}

fn exact_edit_schema(description: &str) -> Value {
    object_schema(
        &["path", "anchor", "target", "content"],
        vec![
            (
                "path",
                path_schema("Editable UTF-8 text file inside current CToolScopeBase."),
            ),
            (
                "anchor",
                string_schema("Exact anchor text. Must match exactly once in the file."),
            ),
            (
                "target",
                string_schema(
                    "Exact target text inside anchor. Must match exactly once inside anchor.",
                ),
            ),
            ("content", string_schema(description)),
        ],
    )
}

fn object_schema(required: &[&str], properties: Vec<(&str, Value)>) -> Value {
    let mut props = Map::new();
    for (name, schema) in properties {
        props.insert(name.to_string(), schema);
    }

    json!({
        "type": "object",
        "additionalProperties": false,
        "required": required,
        "properties": props,
    })
}

fn object_open_schema(description: &str) -> Value {
    json!({
        "type": "object",
        "description": description,
        "additionalProperties": true,
    })
}

fn string_schema(description: &str) -> Value {
    json!({
        "type": "string",
        "description": description,
    })
}

fn path_schema(description: &str) -> Value {
    json!({
        "type": "string",
        "description": description,
        "format": "path",
    })
}

fn integer_schema(description: &str) -> Value {
    json!({
        "type": "integer",
        "description": description,
        "minimum": 0,
    })
}

fn boolean_schema(description: &str) -> Value {
    json!({
        "type": "boolean",
        "description": description,
    })
}

fn enum_schema(values: &[&str], description: &str) -> Value {
    json!({
        "type": "string",
        "description": description,
        "enum": values,
    })
}

fn array_schema(items: Value, description: &str) -> Value {
    json!({
        "type": "array",
        "description": description,
        "items": items,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_serializes_machine_readable_schemas() {
        let spec = CToolSpec {
            name: "ctool_tavily_search_request",
            description: "test",
        };

        let value = serde_json::to_value(spec).unwrap();

        assert_eq!(value["name"], "ctool_tavily_search_request");
        assert_eq!(value["schema_version"], "2026-06-ctool-v1");
        assert_eq!(value["input_schema"]["type"], "object");
        assert_eq!(value["output_schema"]["type"], "object");
        assert_eq!(
            value["input_schema"]["properties"]["action"]["enum"][0],
            "search"
        );
    }

    #[test]
    fn command_request_schema_requires_commands() {
        let schema = ctool_input_schema("ctool_command_request");

        assert_eq!(schema["required"][0], "commands");
        assert_eq!(schema["properties"]["commands"]["type"], "array");
    }

    #[test]
    fn exact_edit_schema_requires_anchor_target_and_content() {
        let schema = ctool_input_schema("ctool_edit_replace_exact");

        assert_eq!(schema["required"][0], "path");
        assert_eq!(schema["required"][1], "anchor");
        assert_eq!(schema["required"][2], "target");
        assert_eq!(schema["required"][3], "content");
    }
    #[test]
    fn all_registered_tools_have_machine_readable_schemas() {
        for spec in crate::registry::available_specs() {
            let input_schema = ctool_input_schema(spec.name);
            let output_schema = ctool_output_schema(spec.name);

            assert_eq!(
                input_schema.get("type").and_then(Value::as_str),
                Some("object"),
                "input schema missing object type for {}",
                spec.name
            );
            assert_eq!(
                output_schema.get("type").and_then(Value::as_str),
                Some("object"),
                "output schema missing object type for {}",
                spec.name
            );
            assert_ne!(
                input_schema.get("description").and_then(Value::as_str),
                Some("Unknown CTool input schema."),
                "missing input schema for {}",
                spec.name
            );
            assert_ne!(
                output_schema.get("description").and_then(Value::as_str),
                Some("Unknown CTool output schema."),
                "missing output schema for {}",
                spec.name
            );
        }
    }
    #[test]
    fn read_tool_schemas_match_public_result_field_names() {
        let list_schema = ctool_output_schema("ctool_list_directory");
        assert!(list_schema["properties"].get("root").is_some());
        assert!(list_schema["properties"].get("total_returned").is_some());
        assert!(list_schema["properties"].get("items").is_some());
        assert!(list_schema["properties"].get("entries").is_none());

        let count_input = ctool_input_schema("ctool_count_matches");
        assert!(count_input["properties"].get("is_regex").is_some());

        let count_output = ctool_output_schema("ctool_count_matches");
        assert!(
            count_output["properties"]
                .get("matching_file_count")
                .is_some()
        );
        assert!(count_output["properties"].get("line_match_count").is_some());
        assert!(count_output["properties"].get("match_count").is_none());

        let extract_input = ctool_input_schema("ctool_extract_lines_matching");
        assert!(extract_input["properties"].get("is_regex").is_some());
        assert!(extract_input["properties"].get("unique").is_some());
        assert!(extract_input["properties"].get("sort").is_some());

        let extract_output = ctool_output_schema("ctool_extract_lines_matching");
        assert!(extract_output["properties"].get("total_returned").is_some());
        assert!(extract_output["properties"].get("lines").is_some());
        assert!(extract_output["properties"].get("line_count").is_none());
    }
    #[test]
    fn edit_and_annotate_schemas_match_public_field_names() {
        let replace_input = ctool_input_schema("ctool_edit_replace");
        assert!(replace_input["properties"].get("old_string").is_some());
        assert!(replace_input["properties"].get("new_string").is_some());
        assert!(replace_input["properties"].get("old").is_none());
        assert!(replace_input["properties"].get("new").is_none());

        let insert_input = ctool_input_schema("ctool_edit_insert");
        assert!(
            insert_input["properties"]
                .get("insert_after_line")
                .is_some()
        );
        assert!(insert_input["properties"].get("line").is_none());

        let preview_input = ctool_input_schema("ctool_preview_diff");
        assert!(preview_input["properties"].get("operations").is_some());
        assert!(preview_input["properties"].get("operation").is_none());

        let exact_output = ctool_output_schema("ctool_edit_replace_exact");
        assert!(exact_output["properties"].get("matched_anchor").is_some());
        assert!(exact_output["properties"].get("matched_target").is_some());
        assert!(exact_output["properties"].get("byte_len_before").is_some());
        assert!(exact_output["properties"].get("anchor_line").is_none());

        let annotate_input = ctool_input_schema("ctool_annotate_markdown");
        assert!(annotate_input["properties"].get("target_text").is_some());
        assert!(
            annotate_input["properties"]
                .get("annotation_kind")
                .is_some()
        );
        assert!(annotate_input["properties"].get("title").is_none());

        let annotate_output = ctool_output_schema("ctool_annotate_markdown");
        assert!(annotate_output["properties"].get("success").is_some());
        assert!(annotate_output["properties"].get("line_number").is_some());
        assert!(
            annotate_output["properties"]
                .get("before_preview")
                .is_some()
        );
        assert!(
            annotate_output["properties"]
                .get("annotation_count")
                .is_none()
        );
    }

    #[test]
    fn file_ops_schemas_match_public_field_names() {
        let delete_file_input = ctool_input_schema("ctool_delete_file");
        assert!(
            delete_file_input["properties"]
                .get("expected_content")
                .is_some()
        );

        let delete_file_output = ctool_output_schema("ctool_delete_file");
        assert!(
            delete_file_output["properties"]
                .get("byte_len_before")
                .is_some()
        );

        let move_file_input = ctool_input_schema("ctool_move_file");
        assert!(move_file_input["properties"].get("overwrite").is_some());

        let move_file_output = ctool_output_schema("ctool_move_file");
        assert!(move_file_output["properties"].get("overwritten").is_some());

        let delete_dir_input = ctool_input_schema("ctool_delete_directory");
        assert!(delete_dir_input["properties"].get("recursive").is_none());
    }
    #[test]
    fn special_tool_output_schemas_match_public_field_names() {
        let command_output = ctool_output_schema("ctool_command_request");
        assert!(command_output["properties"].get("all_success").is_some());
        assert!(command_output["properties"].get("current_dir").is_some());
        assert!(command_output["properties"].get("command_count").is_some());
        assert!(command_output["properties"].get("system_risk").is_some());
        assert!(
            command_output["properties"]
                .get("approval_required")
                .is_some()
        );
        assert!(command_output["properties"].get("commands").is_some());
        assert!(command_output["properties"].get("banner").is_some());
        assert!(command_output["properties"].get("note").is_some());

        let tavily_output = ctool_output_schema("ctool_tavily_search_request");
        assert!(tavily_output["properties"].get("current_dir").is_some());
        assert!(
            tavily_output["properties"]
                .get("suggested_next_step")
                .is_some()
        );
        assert!(tavily_output["properties"].get("user_feedback").is_some());
        assert!(tavily_output["properties"].get("note").is_some());
        assert_eq!(tavily_output["properties"]["action"]["enum"][0], "search");
    }
    #[test]
    fn registry_specs_serialize_with_complete_machine_readable_schema() {
        let specs = crate::registry::available_specs();
        assert!(!specs.is_empty());

        let value = serde_json::to_value(&specs).unwrap();
        let array = value.as_array().unwrap();

        assert_eq!(array.len(), specs.len());

        for item in array {
            let name = item["name"].as_str().unwrap_or_default();
            assert!(!name.is_empty());

            assert_eq!(
                item["schema_version"].as_str(),
                Some("2026-06-ctool-v1"),
                "missing schema_version for {name}"
            );

            assert_eq!(
                item["input_schema"]["type"].as_str(),
                Some("object"),
                "missing input_schema object type for {name}"
            );

            assert_eq!(
                item["output_schema"]["type"].as_str(),
                Some("object"),
                "missing output_schema object type for {name}"
            );

            assert!(
                item["input_schema"].get("properties").is_some(),
                "missing input_schema.properties for {name}"
            );

            assert!(
                item["output_schema"].get("properties").is_some(),
                "missing output_schema.properties for {name}"
            );
        }
    }

    #[test]
    fn registry_schema_exposes_tavily_search_action_enum() {
        let specs = crate::registry::available_specs();
        let value = serde_json::to_value(&specs).unwrap();
        let array = value.as_array().unwrap();

        let tavily = array
            .iter()
            .find(|item| item["name"] == "ctool_tavily_search_request")
            .expect("ctool_tavily_search_request spec missing");

        assert_eq!(
            tavily["input_schema"]["properties"]["action"]["enum"][0],
            "search"
        );
        assert_eq!(
            tavily["output_schema"]["properties"]["action"]["enum"][0],
            "search"
        );
    }

    #[test]
    fn registry_schema_exposes_exact_edit_required_fields() {
        let specs = crate::registry::available_specs();
        let value = serde_json::to_value(&specs).unwrap();
        let array = value.as_array().unwrap();

        let exact = array
            .iter()
            .find(|item| item["name"] == "ctool_edit_replace_exact")
            .expect("ctool_edit_replace_exact spec missing");

        assert_eq!(exact["input_schema"]["required"][0], "path");
        assert_eq!(exact["input_schema"]["required"][1], "anchor");
        assert_eq!(exact["input_schema"]["required"][2], "target");
        assert_eq!(exact["input_schema"]["required"][3], "content");
    }
}
