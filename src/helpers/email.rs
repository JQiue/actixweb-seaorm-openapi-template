use lettre::{
  message::header::ContentType, transport::smtp::authentication::Credentials, Message,
  SmtpTransport, Transport,
};

use crate::config::EnvConfig;

struct SmtpConfig {
  host: &'static str,
  port: u16,
}

enum SmtpService {
  Gmail,
  NetEase126,
  NetEase163,
  QQ,
}

impl SmtpService {
  fn config(&self) -> SmtpConfig {
    match self {
      &SmtpService::QQ => SmtpConfig {
        host: "smtp.qq.com",
        port: 465,
      },
      SmtpService::Gmail => SmtpConfig {
        host: "smtp.gmail.com",
        port: 587,
      },
      SmtpService::NetEase126 => SmtpConfig {
        host: "smtp.126.com",
        port: 25,
      },
      SmtpService::NetEase163 => SmtpConfig {
        host: "smtp.163.com",
        port: 25,
      },
    }
  }
}

pub enum NotifyType {
  Notify,
}

pub struct EmailNotification<'a> {
  pub notify_type: NotifyType,
  pub to_email: &'a str,
  pub subject: &'a str,
  pub body: String,
  pub lang: Option<&'a str>,
}

pub fn send_email_notification(notification: EmailNotification) {
  let to: &str;
  let subject;
  let body;
  match notification.notify_type {
    NotifyType::Notify => {
      to = &notification.to_email;
      subject = "";
      body = "";
      tracing::debug!("Body: {:#?}", body);
    }
  }
  mail(to, &subject, body);
}

pub fn mail(to: &str, subject: &str, body: &str) {
  let EnvConfig {
    smtp_service,
    smtp_host,
    smtp_port,
    smtp_user,
    smtp_pass,
    ..
  } = EnvConfig::load_env().unwrap();
  let host;
  let port;
  if smtp_user.is_none() || smtp_pass.is_none() {
    return;
  }
  if smtp_host.is_some() || smtp_port.is_some() {
    host = smtp_host.unwrap();
    port = smtp_port.unwrap();
  } else if smtp_service.is_some() {
    let smtp_service = match smtp_service.unwrap().as_str() {
      "QQ" => SmtpService::QQ,
      "Gmail" => SmtpService::Gmail,
      "126" => SmtpService::NetEase126,
      "163" => SmtpService::NetEase163,
      _ => {
        tracing::error!("Unsupported SMTP service");
        return;
      }
    };
    host = smtp_service.config().host.to_owned();
    port = smtp_service.config().port;
  } else {
    return;
  }
  let msg = Message::builder()
    .from(
      format!("{} <{}>", "sender", smtp_user.clone().unwrap())
        .parse()
        .unwrap(),
    )
    .to(to.parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_HTML)
    .body(body.to_string())
    .unwrap();
  let mailer = SmtpTransport::relay(&host)
    .unwrap()
    .credentials(Credentials::new(smtp_user.unwrap(), smtp_pass.unwrap()))
    .port(port)
    .build();
  match mailer.send(&msg) {
    Ok(resp) => tracing::info!("{:#?}", resp),
    Err(e) => tracing::error!("Could not send email: {e:?}"),
  }
}
