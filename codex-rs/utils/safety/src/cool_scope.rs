use std::fmt;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoolScope {
    None,
    Workspace,
    SelectedOnly,
    TheEyeofProvidence,
}

impl CoolScope {
    pub fn as_str(self) -> &'static str {
        match self {
            CoolScope::None => "None",
            CoolScope::Workspace => "Workspace",
            CoolScope::SelectedOnly => "SelectedOnly",
            CoolScope::TheEyeofProvidence => "TheEyeofProvidence",
        }
    }
}

impl fmt::Display for CoolScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

static COOL_SCOPE: OnceLock<CoolScope> = OnceLock::new();

pub fn init(scope: CoolScope) {
    COOL_SCOPE
        .set(scope)
        .expect("CoolScope was already initialized");
}

pub fn current() -> CoolScope {
    COOL_SCOPE.get().copied().unwrap_or(CoolScope::Workspace)
}
