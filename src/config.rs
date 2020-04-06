#[derive(PartialEq, Clone, Deserialize)]
#[serde(default)]
pub struct RhisConfig {
    // Log request bodies
    pub request_body: bool,
    // Log response bodies
    pub response_body: bool,
    // Log request headers
    pub request_headers: bool,
    // Log response headers
    pub response_headers: bool,
}

impl Default for RhisConfig {
    fn default() -> Self {
        Self {
            request_body: true,
            response_body: true,
            request_headers: true,
            response_headers: true,
        }
    }
}