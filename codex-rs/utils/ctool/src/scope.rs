use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CToolScopeBase {
    None,
    SelectedOnly,
    CoolWorkspace,
    TheEyeOfProvidence,
}

impl CToolScopeBase {
    pub fn as_str(self) -> &'static str {
        match self {
            CToolScopeBase::None => "none",
            CToolScopeBase::SelectedOnly => "selected-only",
            CToolScopeBase::CoolWorkspace => "cool-workspace",
            CToolScopeBase::TheEyeOfProvidence => "the-eye-of-providence",
        }
    }
}

impl Default for CToolScopeBase {
    fn default() -> Self {
        CToolScopeBase::None
    }
}

impl fmt::Display for CToolScopeBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
