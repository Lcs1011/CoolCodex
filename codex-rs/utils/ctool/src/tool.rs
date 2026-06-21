use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use serde_json::Value;

use crate::context::CToolContext;
use crate::error::CToolResult;
use crate::tool_schema::{ctool_input_schema, ctool_output_schema};

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
