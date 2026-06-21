use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use crate::context::CToolContext;
use crate::error::CToolResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CToolSpec {
    pub name: &'static str,
    pub description: &'static str,
}

impl Serialize for CToolSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let input_schema = ctool_input_schema(self.name);
        let output_schema = ctool_output_schema(self.name);

        let mut state = serializer.serialize_struct("CToolSpec", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("schema_version", "2026-06-ctool-v1")?;
        state.serialize_field("input_schema", &input_schema)?;
        state.serialize_field("output_schema", &output_schema)?;
        state.end()
    }
}

pub trait CTool {
    fn spec(&self) -> CToolSpec;

    fn run_json(&self, ctx: &CToolContext, input: Value) -> CToolResult<Value>;
}

pub fn ctool_input_schema(name: &str) -> Value {
    match name {
        "ctool_command_request" => object_schema(
            &["commands"],
            vec![
                (
                    "commands",
                    array_schema(
                        string_schema("Command line to preview and possibly execute."),
                        "Ordered command list. Each command is evaluated by command.toml and hard-risk rules.",
                    ),
                ),
                (
                    "ai_risk_upgrade",
                    enum_schema(
                        &["green", "yellow", "red", "blocked"],
                        "Optional AI-side risk upgrade. Cannot lower system risk.",
                    ),
                ),
                ("reason", string_schema("Optional request reason shown in audit output.")),
                (
                    "yellow_confirmation",
                    string_schema("Required only when final risk is YELLOW and user approves or rejects."),
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
                    enum_schema(
                        &["search"],
                        "V1 supports only public text search. extract/zoom/research/images are intentionally blocked.",
                    ),
                ),
                ("query", string_schema("Public web search query. Do not include local file contents or secrets.")),
                (
                    "file_name_hint",
                    string_schema("Optional safe filename hint for the cached Markdown result."),
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
                ("path", path_schema("Directory path inside current CToolScopeBase.")),
                ("max_depth", integer_schema("Maximum recursion depth. Hard capped by tool.")),
                ("max_entries", integer_schema("Maximum returned entries. Hard capped by tool.")),
                ("include_hidden", boolean_schema("Whether to include hidden files and directories.")),
            ],
        ),

        "ctool_rg_search" => object_schema(
            &["path", "query"],
            vec![
                ("path", path_schema("Directory or file path inside current CToolScopeBase.")),
                ("query", string_schema("Literal text query searched across UTF-8 files.")),
                ("case_sensitive", boolean_schema("Whether matching is case-sensitive.")),
                ("max_depth", integer_schema("Maximum directory recursion depth.")),
                ("max_results", integer_schema("Maximum returned matches.")),
                ("include_hidden", boolean_schema("Whether to search hidden files and directories.")),
            ],
        ),

        "ctool_rg_search_context" => object_schema(
            &["path", "query"],
            vec![
                ("path", path_schema("Directory or file path inside current CToolScopeBase.")),
                ("query", string_schema("Literal text query searched across UTF-8 files.")),
                ("before", integer_schema("Number of context lines before each match.")),
                ("after", integer_schema("Number of context lines after each match.")),
                ("case_sensitive", boolean_schema("Whether matching is case-sensitive.")),
                ("max_depth", integer_schema("Maximum directory recursion depth.")),
                ("max_results", integer_schema("Maximum returned matches.")),
                ("include_hidden", boolean_schema("Whether to search hidden files and directories.")),
            ],
        ),

        "ctool_regex_search" => object_schema(
            &["path", "pattern"],
            vec![
                ("path", path_schema("Directory or file path inside current CToolScopeBase.")),
                ("pattern", string_schema("Rust regex pattern searched across UTF-8 files.")),
                ("case_sensitive", boolean_schema("Whether matching is case-sensitive.")),
                ("max_depth", integer_schema("Maximum directory recursion depth.")),
                ("max_results", integer_schema("Maximum returned matches.")),
                ("include_hidden", boolean_schema("Whether to search hidden files and directories.")),
            ],
        ),

        "ctool_count_matches" => object_schema(
            &["path", "query"],
            vec![
                ("path", path_schema("Directory or file path inside current CToolScopeBase.")),
                ("query", string_schema("Literal text or regex pattern to count.")),
                ("is_regex", boolean_schema("Whether query is interpreted as a Rust regex pattern.")),
                ("case_sensitive", boolean_schema("Whether matching is case-sensitive.")),
                ("max_depth", integer_schema("Maximum directory recursion depth.")),
                ("include_hidden", boolean_schema("Whether to search hidden files and directories.")),
            ],
        ),

        "ctool_extract_lines_matching" => object_schema(
            &["path", "query"],
            vec![
                ("path", path_schema("Directory or file path inside current CToolScopeBase.")),
                ("query", string_schema("Literal text or regex pattern used to extract matching lines.")),
                ("is_regex", boolean_schema("Whether query is interpreted as a Rust regex pattern.")),
                ("case_sensitive", boolean_schema("Whether matching is case-sensitive.")),
                ("unique", boolean_schema("Whether duplicate extracted lines are removed.")),
                ("sort", boolean_schema("Whether extracted lines are sorted.")),
                ("max_depth", integer_schema("Maximum directory recursion depth.")),
                ("max_results", integer_schema("Maximum returned lines.")),
                ("include_hidden", boolean_schema("Whether to search hidden files and directories.")),
            ],
        ),

        "ctool_read_file" => object_schema(
            &["path"],
            vec![
                ("path", path_schema("UTF-8 text file path inside current CToolScopeBase.")),
                ("max_bytes", integer_schema("Maximum bytes to read. Hard capped by tool.")),
            ],
        ),

        "ctool_read_code_range" => object_schema(
            &["path", "start_line", "end_line"],
            vec![
                ("path", path_schema("UTF-8 text file path inside current CToolScopeBase.")),
                ("start_line", integer_schema("1-based inclusive start line.")),
                ("end_line", integer_schema("1-based inclusive end line.")),
            ],
        ),

        "ctool_tail_file" => object_schema(
            &["path"],
            vec![
                ("path", path_schema("UTF-8 text file path inside current CToolScopeBase.")),
                ("line_count", integer_schema("Number of trailing lines to read.")),
            ],
        ),

        "ctool_edit_replace" => object_schema(
            &["path", "old_string", "new_string"],
            vec![
                ("path", path_schema("Editable UTF-8 text file inside current CToolScopeBase.")),
                ("old_string", string_schema("Exact old text to replace. Must match exactly once.")),
                ("new_string", string_schema("Replacement text.")),
            ],
        ),

        "ctool_edit_insert" => object_schema(
            &["path", "insert_after_line", "content"],
            vec![
                ("path", path_schema("Editable UTF-8 text file inside current CToolScopeBase.")),
                ("insert_after_line", integer_schema("0 inserts at file beginning; N inserts after line N.")),
                ("content", string_schema("Text to insert. A final newline is added when missing.")),
            ],
        ),

        "ctool_preview_diff" => object_schema(
            &["path", "operations"],
            vec![
                ("path", path_schema("Editable UTF-8 text file inside current CToolScopeBase.")),
                (
                    "operations",
                    array_schema(
                        object_open_schema("Preview operation: { operation: replace, old_string, new_string } or { operation: insert, insert_after_line, content }."),
                        "Ordered preview operations. Nothing is written.",
                    ),
                ),
            ],
        ),

        "ctool_edit_batch" => object_schema(
            &["operations"],
            vec![
                (
                    "operations",
                    array_schema(
                        object_open_schema("One edit operation."),
                        "Ordered edit operations applied as a batch.",
                    ),
                ),
            ],
        ),

        "ctool_edit_replace_exact" => exact_edit_schema("Replace target text inside a unique anchor."),
        "ctool_edit_insert_before_exact" => exact_edit_schema("Insert content before target text inside a unique anchor."),
        "ctool_edit_insert_after_exact" => exact_edit_schema("Insert content after target text inside a unique anchor."),
        "ctool_edit_remove_exact" => object_schema(
            &["path", "anchor", "target"],
            vec![
                ("path", path_schema("Editable UTF-8 text file inside current CToolScopeBase.")),
                ("anchor", string_schema("Exact anchor text. Must match exactly once in the file.")),
                ("target", string_schema("Exact target text inside anchor. Must match exactly once inside anchor.")),
            ],
        ),

        "ctool_create_file" => object_schema(
            &["path", "content"],
            vec![
                ("path", path_schema("New file path inside current CToolScopeBase.")),
                ("content", string_schema("UTF-8 file content to write.")),
                ("overwrite", boolean_schema("Whether existing file may be overwritten, if supported by tool.")),
            ],
        ),

        "ctool_delete_file" => object_schema(
            &["path"],
            vec![
                ("path", path_schema("File path inside current CToolScopeBase to delete.")),
                ("expected_content", string_schema("Optional exact file content guard. Delete is refused if content differs.")),
            ],
        ),

        "ctool_move_file" => object_schema(
            &["from", "to"],
            vec![
                ("from", path_schema("Source file path inside current CToolScopeBase.")),
                ("to", path_schema("Destination file path inside current CToolScopeBase.")),
                ("overwrite", boolean_schema("Whether an existing destination file may be overwritten.")),
            ],
        ),

        "ctool_create_directory" => object_schema(
            &["path"],
            vec![("path", path_schema("Directory path inside current CToolScopeBase to create."))],
        ),

        "ctool_delete_directory" => object_schema(
            &["path"],
            vec![
                ("path", path_schema("Empty directory path inside current CToolScopeBase to delete. Recursive deletion is never supported.")),
            ],
        ),

        "ctool_move_directory" => object_schema(
            &["from", "to"],
            vec![
                ("from", path_schema("Source directory path inside current CToolScopeBase.")),
                ("to", path_schema("Destination directory path inside current CToolScopeBase.")),
            ],
        ),

        "ctool_annotate_markdown" => object_schema(
            &["path", "target_text"],
            vec![
                ("path", path_schema("Markdown file path inside current CToolScopeBase.")),
                ("target_text", string_schema("Exact Markdown text to wrap with a safe <mark> annotation.")),
                ("annotation_kind", enum_schema(&["normal", "important"], "Annotation color/intensity.")),
                ("annotation_direction", enum_schema(&["up", "down"], "Optional arrow prefix inserted inside the mark.")),
                ("occurrence", integer_schema("1-based occurrence when target_text appears more than once.")),
                ("allow_readonly", boolean_schema("Reserved compatibility flag. Write scope is still enforced.")),
                ("dry_run", boolean_schema("Preview annotation without writing the file.")),
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
                ("will_execute", boolean_schema("Whether the request will execute in this call.")),
                ("executed", boolean_schema("Whether commands executed.")),
                ("blocked", boolean_schema("Whether the request was blocked.")),
                ("rejected", boolean_schema("Whether the user rejected confirmation.")),
                ("all_success", boolean_schema("Whether all executed commands exited successfully. Null when not executed.")),
                ("result_file", string_schema("Optional Markdown result file path.")),
                ("log_file", string_schema("Optional audit log file path.")),
                ("current_dir", string_schema("Command execution current directory.")),
                ("command_count", integer_schema("Number of requested commands.")),
                ("system_risk", enum_schema(&["GREEN", "YELLOW", "RED", "BLOCKED"], "Risk before AI-side upgrade.")),
                ("ai_risk_upgrade", enum_schema(&["GREEN", "YELLOW", "RED", "BLOCKED"], "Optional AI-side risk upgrade.")),
                ("final_risk", enum_schema(&["GREEN", "YELLOW", "RED", "BLOCKED"], "Final risk after all rules.")),
                ("approval_required", string_schema("Approval mode: auto, once, twice, or blocked.")),
                ("request_reason", string_schema("Optional user/AI reason for the request.")),
                ("user_feedback", string_schema("Optional rejection feedback from user confirmation input.")),
                ("commands", array_schema(object_open_schema("Command preview item with command, risk, and reason."), "Per-command risk preview list.")),
                ("display_text", string_schema("Human-readable preview text.")),
                ("banner", string_schema("Human-readable risk banner.")),
                ("note", string_schema("Human-readable execution/block/rejection note.")),
            ],
        ),

        "ctool_tavily_search_request" => object_schema(
            &[],
            vec![
                ("will_execute", boolean_schema("Whether the request will execute in this call.")),
                ("executed", boolean_schema("Whether Tavily was called.")),
                ("blocked", boolean_schema("Whether the request was blocked.")),
                ("rejected", boolean_schema("Whether the user rejected confirmation.")),
                ("current_dir", string_schema("Current CoolWorkspace path.")),
                ("action", enum_schema(&["search"], "V1 action label. Only search is executable.")),
                ("final_risk", enum_schema(&["GREEN", "YELLOW", "RED", "BLOCKED"], "Final risk label.")),
                ("output_file", string_schema("Cached Markdown result path.")),
                ("log_file", string_schema("Audit log path.")),
                ("summary", string_schema("Short summary from cached Markdown.")),
                ("suggested_next_step", string_schema("Suggested next action for the AI/user.")),
                ("user_feedback", string_schema("Optional rejection feedback from user confirmation input.")),
                ("note", string_schema("Human-readable execution/block/rejection note.")),
            ],
        ),

        "ctool_list_directory" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved directory root path.")),
                ("total_returned", integer_schema("Number of returned entries.")),
                ("truncated", boolean_schema("Whether results were truncated.")),
                ("items", array_schema(object_open_schema("Directory entry with path, kind, and optional byte_len."), "Directory entries.")),
            ],
        ),

        "ctool_rg_search" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved search root path.")),
                ("query", string_schema("Literal query used for search.")),
                ("total_returned", integer_schema("Number of matches returned.")),
                ("truncated", boolean_schema("Whether results were truncated.")),
                ("matches", array_schema(object_open_schema("Search match with path, line_number, and line."), "Search matches.")),
            ],
        ),

        "ctool_regex_search" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved search root path.")),
                ("pattern", string_schema("Rust regex pattern used for search.")),
                ("total_returned", integer_schema("Number of matches returned.")),
                ("truncated", boolean_schema("Whether results were truncated.")),
                ("matches", array_schema(object_open_schema("Regex match with path, line_number, and line."), "Regex matches.")),
            ],
        ),

        "ctool_rg_search_context" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved search root path.")),
                ("query", string_schema("Literal query used for search.")),
                ("total_returned", integer_schema("Number of matches returned.")),
                ("truncated", boolean_schema("Whether results were truncated.")),
                ("matches", array_schema(object_open_schema("Search match with path, line_number, and context lines."), "Search matches.")),
            ],
        ),

        "ctool_count_matches" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved search root path.")),
                ("query", string_schema("Literal query or regex pattern used for counting.")),
                ("is_regex", boolean_schema("Whether query was interpreted as a Rust regex pattern.")),
                ("file_count", integer_schema("Number of scanned files.")),
                ("matching_file_count", integer_schema("Number of files containing matches.")),
                ("line_match_count", integer_schema("Number of matching lines.")),
            ],
        ),

        "ctool_extract_lines_matching" => object_schema(
            &[],
            vec![
                ("root", string_schema("Resolved search root path.")),
                ("query", string_schema("Literal query or regex pattern used for extraction.")),
                ("is_regex", boolean_schema("Whether query was interpreted as a Rust regex pattern.")),
                ("total_returned", integer_schema("Number of returned lines.")),
                ("truncated", boolean_schema("Whether results were truncated.")),
                ("lines", array_schema(object_open_schema("Extracted line with path, line_number, and line."), "Extracted lines.")),
            ],
        ),

        "ctool_read_file" => object_schema(
            &[],
            vec![
                ("path", string_schema("Resolved file path.")),
                ("byte_len", integer_schema("File byte length.")),
                ("truncated", boolean_schema("Whether content was truncated.")),
                ("content", string_schema("UTF-8 file content.")),
            ],
        ),

        "ctool_read_code_range" => object_schema(
            &[],
            vec![
                ("path", string_schema("Resolved file path.")),
                ("start_line", integer_schema("1-based inclusive start line.")),
                ("end_line", integer_schema("1-based inclusive end line.")),
                ("content", string_schema("UTF-8 code range content.")),
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
                ("replaced", integer_schema("Number of replacements. This tool requires exactly one replacement.")),
                ("byte_len_before", integer_schema("File byte length before edit.")),
                ("byte_len_after", integer_schema("File byte length after edit.")),
            ],
        ),

        "ctool_edit_insert" => object_schema(
            &[],
            vec![
                ("path", string_schema("Edited file path.")),
                ("inserted_after_line", integer_schema("Line after which content was inserted. 0 means file beginning.")),
                ("byte_len_before", integer_schema("File byte length before edit.")),
                ("byte_len_after", integer_schema("File byte length after edit.")),
            ],
        ),

        "ctool_preview_diff" => object_schema(
            &[],
            vec![
                ("path", string_schema("Target file path.")),
                ("operation_count", integer_schema("Number of preview operations.")),
                ("changed", boolean_schema("Whether operation would change the file.")),
                ("diff", string_schema("Simple unified diff preview.")),
            ],
        ),

        "ctool_edit_batch" => object_schema(
            &[],
            vec![
                ("operation_count", integer_schema("Number of applied edit operations.")),
                ("files_touched", array_schema(string_schema("Edited file path."), "Files written by the batch.")),
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
                ("matched_anchor", integer_schema("Number of matched anchors. Successful runs return 1.")),
                ("matched_target", integer_schema("Number of matched targets inside the anchor. Successful runs return 1.")),
                ("byte_len_before", integer_schema("File byte length before edit.")),
                ("byte_len_after", integer_schema("File byte length after edit.")),
            ],
        ),

        "ctool_annotate_markdown" => object_schema(
            &[],
            vec![
                ("success", boolean_schema("Whether annotation planning/execution succeeded.")),
                ("dry_run", boolean_schema("Whether the file was left unchanged.")),
                ("readonly_exception_used", boolean_schema("Whether readonly exception was used.")),
                ("annotation_kind", enum_schema(&["normal", "important"], "Annotation kind used.")),
                ("annotation_direction", enum_schema(&["up", "down"], "Optional arrow direction used.")),
                ("path", string_schema("Annotated Markdown file path.")),
                ("line_number", integer_schema("1-based line number of the annotation target.")),
                ("occurrence", integer_schema("1-based selected occurrence.")),
                ("before_preview", string_schema("Preview before annotation.")),
                ("after_preview", string_schema("Preview after annotation.")),
                ("note", string_schema("Human-readable result note.")),
            ],
        ),

        "ctool_create_file" => object_schema(
            &[],
            vec![
                ("path", string_schema("Created file path.")),
                ("byte_len", integer_schema("Written byte length.")),
                ("overwritten", boolean_schema("Whether an existing file was overwritten.")),
            ],
        ),

        "ctool_delete_file" => object_schema(
            &[],
            vec![
                ("path", string_schema("Deleted file path.")),
                ("byte_len_before", integer_schema("File byte length before deletion.")),
                ("deleted", boolean_schema("Whether file was deleted.")),
            ],
        ),

        "ctool_move_file" => object_schema(
            &[],
            vec![
                ("from", string_schema("Resolved source file path.")),
                ("to", string_schema("Resolved destination file path.")),
                ("overwritten", boolean_schema("Whether an existing destination file was overwritten.")),
                ("moved", boolean_schema("Whether file was moved.")),
            ],
        ),

        "ctool_create_directory" => object_schema(
            &[],
            vec![
                ("path", string_schema("Created directory path.")),
                ("created", boolean_schema("Whether directory was newly created.")),
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

        _ => object_open_schema("Unknown CTool output schema."),
    }
}

fn exact_edit_schema(description: &str) -> Value {
    object_schema(
        &["path", "anchor", "target", "content"],
        vec![
            ("path", path_schema("Editable UTF-8 text file inside current CToolScopeBase.")),
            ("anchor", string_schema("Exact anchor text. Must match exactly once in the file.")),
            ("target", string_schema("Exact target text inside anchor. Must match exactly once inside anchor.")),
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
        assert_eq!(value["input_schema"]["properties"]["action"]["enum"][0], "search");
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
        assert!(count_output["properties"].get("matching_file_count").is_some());
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
        assert!(insert_input["properties"].get("insert_after_line").is_some());
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
        assert!(annotate_input["properties"].get("annotation_kind").is_some());
        assert!(annotate_input["properties"].get("title").is_none());

        let annotate_output = ctool_output_schema("ctool_annotate_markdown");
        assert!(annotate_output["properties"].get("success").is_some());
        assert!(annotate_output["properties"].get("line_number").is_some());
        assert!(annotate_output["properties"].get("before_preview").is_some());
        assert!(annotate_output["properties"].get("annotation_count").is_none());
    }

    #[test]
    fn file_ops_schemas_match_public_field_names() {
        let delete_file_input = ctool_input_schema("ctool_delete_file");
        assert!(delete_file_input["properties"].get("expected_content").is_some());

        let delete_file_output = ctool_output_schema("ctool_delete_file");
        assert!(delete_file_output["properties"].get("byte_len_before").is_some());

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
        assert!(command_output["properties"].get("approval_required").is_some());
        assert!(command_output["properties"].get("commands").is_some());
        assert!(command_output["properties"].get("banner").is_some());
        assert!(command_output["properties"].get("note").is_some());

        let tavily_output = ctool_output_schema("ctool_tavily_search_request");
        assert!(tavily_output["properties"].get("current_dir").is_some());
        assert!(tavily_output["properties"].get("suggested_next_step").is_some());
        assert!(tavily_output["properties"].get("user_feedback").is_some());
        assert!(tavily_output["properties"].get("note").is_some());
        assert_eq!(
            tavily_output["properties"]["action"]["enum"][0],
            "search"
        );
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
