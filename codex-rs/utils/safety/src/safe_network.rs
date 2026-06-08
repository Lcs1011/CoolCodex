use crate::safe_mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkPurpose {
    ChatGPTAuth,
    ModelApi,
    Other,
}

impl NetworkPurpose {
    pub fn allowed_in_safe_mode(self) -> bool {
        matches!(self, NetworkPurpose::ChatGPTAuth | NetworkPurpose::ModelApi)
    }
}

pub async fn send(
    purpose: NetworkPurpose,
    request: reqwest::RequestBuilder,
) -> anyhow::Result<reqwest::Response> {
    if safe_mode::enabled() && !purpose.allowed_in_safe_mode() {
        anyhow::bail!("network request blocked by SafeMode: {purpose:?}");
    }
    Ok(request.send().await?)
}
