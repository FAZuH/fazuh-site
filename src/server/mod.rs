use std::collections::HashMap;

use dioxus::prelude::*;
#[cfg(feature = "server")]
use http::HeaderMap;
use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "server")]
mod config;
#[cfg(feature = "server")]
mod logging;
#[cfg(feature = "server")]
mod rate_limit;
#[cfg(feature = "server")]
mod smtp;

#[cfg(feature = "server")]
pub use logging::init_logging;

#[cfg(feature = "server")]
use crate::validation;
use crate::validation::ContactForm;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContactResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Vec<String>>>,
}

#[server]
pub async fn submit_contact(form: ContactForm) -> Result<ContactResponse, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use tracing::error;
        use tracing::info;
        use tracing::warn;
        use validator::Validate;

        let headers: HeaderMap = dioxus::fullstack::FullstackContext::extract().await?;
        let ip = get_client_ip(&headers);

        if let Err(errors) = form.validate() {
            warn!("Form validation failed for IP {ip}");
            return Ok(ContactResponse {
                success: false,
                message: "Please fix the form errors and try again.".to_string(),
                errors: Some(validation::format_errors(&errors)),
            });
        }

        if !rate_limit::check_contact_rate_limit(&ip) {
            warn!("Rate limit exceeded for IP {ip}");
            return Ok(ContactResponse {
                success: false,
                message: "Too many requests. Please try again later.".to_string(),
                errors: None,
            });
        }

        match smtp::send_email(&form).await {
            Ok(()) => {
                info!("Message sent successfully from IP {ip}");
                Ok(ContactResponse {
                    success: true,
                    message: "Message sent successfully!".to_string(),
                    errors: None,
                })
            }
            Err(e) => {
                error!("Failed to send message from IP {ip}: {e}");
                Ok(ContactResponse {
                    success: false,
                    message: "Failed to send message. Please try again later.".to_string(),
                    errors: None,
                })
            }
        }
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = form;
        unreachable!()
    }
}

#[cfg(feature = "server")]
/// Extracts the client IP from request headers.
///
/// Relies on `X-Forwarded-For` (first address) with fallback to `X-Real-Ip`.
/// **Security**: Ensure your reverse proxy strips these headers from external
/// requests before setting its own trusted values, otherwise clients can spoof
/// their IP and bypass the per-IP rate limit.
fn get_client_ip(headers: &HeaderMap) -> String {
    headers
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            headers
                .get("X-Real-Ip")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.trim().to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(feature = "server")]
#[cfg(test)]
mod tests {
    use http::HeaderMap;
    use http::HeaderValue;

    use super::*;

    #[test]
    fn extracts_ip_from_x_forwarded_for_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Forwarded-For",
            HeaderValue::from_static("1.2.3.4, 5.6.7.8"),
        );

        let result = get_client_ip(&headers);

        assert_eq!(result, "1.2.3.4");
    }

    #[test]
    fn extracts_ip_from_x_real_ip_header() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Real-Ip", HeaderValue::from_static("9.8.7.6"));

        let result = get_client_ip(&headers);

        assert_eq!(result, "9.8.7.6");
    }

    #[test]
    fn prefers_x_forwarded_for_over_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Forwarded-For", HeaderValue::from_static("1.1.1.1"));
        headers.insert("X-Real-Ip", HeaderValue::from_static("2.2.2.2"));

        let result = get_client_ip(&headers);

        assert_eq!(result, "1.1.1.1");
    }

    #[test]
    fn returns_unknown_when_no_ip_headers_present() {
        let headers = HeaderMap::new();

        let result = get_client_ip(&headers);

        assert_eq!(result, "unknown");
    }

    #[test]
    fn trims_whitespace_from_ip_value() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Forwarded-For",
            HeaderValue::from_static("  192.168.1.1  "),
        );

        let result = get_client_ip(&headers);

        assert_eq!(result, "192.168.1.1");
    }
}
