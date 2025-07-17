use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdatePinRequest {
    pub pin: String,
}
