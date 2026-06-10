use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CToolScopeBase {
    None,
    CoolWorkspace,
    SelectedOnly,
    TheEyeofProvidence,
}

impl CToolScopeBase {
    pub fn as_str(self) -> &'static str {
        match self {
            CToolScopeBase::None => "None",
            CToolScopeBase::CoolWorkspace => "CoolWorkspace",
            CToolScopeBase::SelectedOnly => "SelectedOnly",
            CToolScopeBase::TheEyeofProvidence => "TheEyeofProvidence",
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
