#[cfg(feature = "server")]
use lettre::AsyncSmtpTransport;
#[cfg(feature = "server")]
use lettre::AsyncTransport;
#[cfg(feature = "server")]
use lettre::Message;
#[cfg(feature = "server")]
use lettre::Tokio1Executor;
#[cfg(feature = "server")]
use lettre::message::Mailbox;
#[cfg(feature = "server")]
use lettre::transport::smtp::authentication::Credentials;
#[cfg(feature = "server")]
use tracing::error;

#[cfg(feature = "server")]
use crate::utils::config::Config;
#[cfg(feature = "server")]
use crate::utils::validation::ContactForm;

#[cfg(feature = "server")]
pub async fn send_email(form: &ContactForm) -> Result<(), String> {
    let config = Config::get();

    let from: Mailbox = config
        .smtp_from
        .parse()
        .map_err(|e| format!("Invalid SMTP_FROM address: {e}"))?;
    let to: Mailbox = config
        .smtp_to
        .parse()
        .map_err(|e| format!("Invalid SMTP_TO address: {e}"))?;

    let email = Message::builder()
        .from(from)
        .to(to)
        .reply_to(format!("{} <{}>", form.name, form.email).parse().unwrap())
        .subject(format!("[fazuh-site] Message from {}", form.name))
        .header(lettre::message::header::ContentType::TEXT_PLAIN)
        .body(format!(
            "From: {name} <{email}>\n\n{message}",
            name = form.name,
            email = form.email,
            message = form.message,
        ))
        .map_err(|e| format!("Failed to build email: {e}"))?;

    let transport = if config.smtp_password.is_empty() {
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)
            .map_err(|e| format!("Failed to configure SMTP relay: {e}"))?
            .port(config.smtp_port)
            .build()
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)
            .map_err(|e| format!("Failed to configure SMTP relay: {e}"))?
            .port(config.smtp_port)
            .credentials(Credentials::new(
                config.smtp_username.clone(),
                config.smtp_password.clone(),
            ))
            .build()
    };

    match transport.send(email).await {
        Ok(_response) => Ok(()),
        Err(e) => {
            error!("Failed to send email via SMTP: {e}");
            Err(format!("SMTP send failed: {e}"))
        }
    }
}

#[cfg(not(feature = "server"))]
pub async fn send_email(_form: &crate::utils::validation::ContactForm) -> Result<(), String> {
    unreachable!("send_email called without server feature")
}
