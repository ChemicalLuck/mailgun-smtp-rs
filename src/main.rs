mod cli;
mod recipients;
mod replace;

use std::fs;

use clap::Parser;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use cli::Cli;
use recipients::Recipients;
use replace::replace_variables;
use serde::Serialize;

#[derive(Serialize)]
struct Output {
    email: String,
    sent: bool,
}

fn send(settings: Settings, recipients: Recipients, subject: String, content: String) {
    let creds = Credentials::new(settings.username, settings.password);

    let mailer = SmtpTransport::starttls_relay(&settings.smtp_relay)
        .unwrap()
        .credentials(creds)
        .build();

    let recipient_count = recipients.len() as u64;
    let progress_bar = indicatif::ProgressBar::new(recipient_count);
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    let mut sent = 0;

    for mut recipient in recipients {
        progress_bar.set_message(format!("{}: {}", &recipient.email, "Sending..."));

        let body = match replace_variables(&content, &recipient.variables) {
            Ok(body) => body,
            Err(e) => {
                progress_bar.set_message(format!("{}: {}", &recipient.email, e));
                progress_bar.inc(1);
                continue;
            }
        };

        let subject = match replace_variables(&subject, &recipient.variables) {
            Ok(subject) => subject,
            Err(e) => {
                progress_bar.set_message(format!("{}: {}", &recipient.email, e));
                progress_bar.inc(1);
                continue;
            }
        };

        let email = Message::builder()
            .from(settings.from.clone())
            .reply_to(settings.reply_to.clone())
            .to(recipient.email.clone())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            .unwrap();

        match mailer.send(&email) {
            Ok(_) => {
                progress_bar.set_message(format!("{}: {}", &recipient.email, "Sent"));
                recipient.sent = true;
                sent += 1;
            }
            Err(e) => {
                progress_bar.set_message(format!("{}: {}", &recipient.email, e));
            }
        }

        writer
            .serialize(Output {
                email: recipient.email.to_string(),
                sent: recipient.sent,
            })
            .unwrap();

        progress_bar.inc(1);
    }

    progress_bar.finish_with_message(format!("Sent {}/{}", sent, recipient_count));
    writer.flush().unwrap();
}

fn preview(recipients: Recipients, subject: String, content: String) {
    let recipient_count = recipients.len() as u64;

    for (i, recipient) in recipients.into_iter().enumerate() {
        let body = match replace_variables(&content, &recipient.variables) {
            Ok(body) => body,
            Err(e) => {
                println!("{}: {}", &recipient.email, e);
                continue;
            }
        };

        let subject = match replace_variables(&subject, &recipient.variables) {
            Ok(subject) => subject,
            Err(e) => {
                println!("{}: {}", &recipient.email, e);
                continue;
            }
        };

        println!("TO:   {}", recipient.email.email.to_string());
        println!("SUBJECT: {}", subject);
        println!("BODY:\n{}", body);
        println!("Previewing {} of {}", i + 1, recipient_count);
        println!("Press enter to continue...");

        // wait for enter to be pressed
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}

struct Settings {
    username: String,
    password: String,
    smtp_relay: String,
    from: Mailbox,
    reply_to: Mailbox,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    let content = fs::read_to_string(&cli.message)?;

    let recipients = Recipients::from_path(&cli.recipients)?;

    match cli.command {
        cli::Command::Send {
            username,
            password,
            smtp_relay,
            from,
            reply_to,
        } => {
            let settings = Settings {
                username: username.unwrap(),
                password: password.unwrap(),
                smtp_relay: smtp_relay.unwrap(),
                from: from.unwrap(),
                reply_to: reply_to.unwrap(),
            };
            send(settings, recipients, cli.subject, content);
        }
        cli::Command::Preview {} => {
            preview(recipients, cli.subject, content);
        }
    }
    Ok(())
}
