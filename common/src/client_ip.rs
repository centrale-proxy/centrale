use serde_derive::{Deserialize, Serialize};

/// All the different representations of the client address, each as its own
/// named field. `None` means that source wasn't present in the request.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ClientIP {
    /// Chain from the standardized `Forwarded` header (RFC 7239).
    /// Multiple `for=` values are joined with ", " to keep it a single string.
    pub forwarded: Option<String>,
    /// Raw value of the legacy `X-Forwarded-For` header.
    pub x_forwarded_for: Option<String>,
    /// Value of `X-Real-IP`, set by some proxies (e.g. nginx).
    pub x_real_ip: Option<String>,
    /// The direct peer address of the TCP connection.
    pub client_addr: Option<String>,
}
///
impl ClientIP {
    /// Best single value for logging, in order of preference.
    /// Prefers the direct TCP peer (`client_addr`), falling back to the
    /// forwarded headers. Those headers are client-supplied and can be
    /// spoofed unless a trusted proxy overwrites them.
    pub fn for_logging(&self) -> &str {
        self.client_addr
            .as_deref()
            .or(self.forwarded.as_deref())
            .or(self.x_forwarded_for.as_deref())
            .or(self.x_real_ip.as_deref())
            .unwrap_or("-")
    }
}
