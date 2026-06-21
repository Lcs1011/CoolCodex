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
                ("query", string_schema("Literal text query to count.")),
                ("case_sensitive", boolean_schema("Whether matching is case-sensitive.")),
                ("max_depth", integer_schema("Maximum directory recursion depth.")),
                ("include_hidden", boolean_schema("Whether to search hidden files and directories.")),
            ],
        ),

        "ctool_extract_lines_matching" => object_schema(
            &["path", "query"],
            vec![
                ("path", path_schema("Directory or file path inside current CToolScopeBase.")),
                ("query", string_schema("Literal text query used to extract matching lines.")),
                ("case_sensitive", boolean_schema("Whether matching is case-sensitive.")),
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
            &["path", "old", "new"],
            vec![
                ("path", path_schema("Editable UTF-8 text file inside current CToolScopeBase.")),
                ("old", string_schema("Exact old text to replace. Must match uniquely according to tool rules.")),
                ("new", string_schema("Replacement text.")),
            ],
        ),

        "ctool_edit_insert" => object_schema(
            &["path", "line", "content"],
            vec![
                ("path", path_schema("Editable UTF-8 text file inside current CToolScopeBase.")),
                ("line", integer_schema("1-based insertion line.")),
                ("content", string_schema("Text to insert.")),
            ],
        ),

        "ctool_preview_diff" => object_schema(
            &["path", "operation"],
            vec![
                ("path", path_schema("Editable UTF-8 text file inside current CToolScopeBase.")),
                (
                    "operation",
                    object_open_schema("Diff preview operation. Uses the same payload style as edit tools."),
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
            vec![("path", path_schema("File path inside current CToolScopeBase to delete."))],
        ),

        "ctool_move_file" => object_schema(
            &["from", "to"],
            vec![
                ("from", path_schema("Source file path inside current CToolScopeBase.")),
                ("to", path_schema("Destination file path inside current CToolScopeBase.")),
            ],
        ),

        "ctool_create_directory" => object_schema(
            &["path"],
            vec![("path", path_schema("Directory path inside current CToolScopeBase to create."))],
        ),

        "ctool_delete_directory" => object_schema(
            &["path"],
            vec![
                ("path", path_schema("Directory path inside current CToolScopeBase to delete.")),
                ("recursive", boolean_schema("Whether recursive deletion is allowed, if supported by tool.")),
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
            &["path"],
            vec![
                ("path", path_schema("Markdown file path inside current CToolScopeBase.")),
                ("title", string_schema("Optional annotation title.")),
                ("content", string_schema("Annotation content.")),
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
                ("final_risk", string_schema("Final risk label.")),
                ("result_file", string_schema("Optional result file path.")),
                ("log_file", string_schema("Optional audit log file path.")),
                ("display_text", string_schema("Human-readable preview text.")),
            ],
        ),
        "ctool_tavily_search_request" => object_schema(
            &[],
            vec![
                ("will_execute", boolean_schema("Whether the request will execute in this call.")),
                ("executed", boolean_schema("Whether Tavily was called.")),
                ("blocked", boolean_schema("Whether the request was blocked.")),
                ("rejected", boolean_schema("Whether the user rejected confirmation.")),
                ("action", string_schema("Action label.")),
                ("final_risk", string_schema("Final risk label.")),
                ("output_file", string_schema("Cached Markdown result path.")),
                ("log_file", string_schema("Audit log path.")),
                ("summary", string_schema("Short summary from cached Markdown.")),
            ],
        ),
        _ => object_open_schema("Tool output object. See concrete tool result fields for exact details."),
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
}
