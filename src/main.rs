mod cli;
mod email;
mod recipients;

use std::collections::HashMap;
use std::{
    fs,
    io::{BufRead, BufReader},
};

use chrono::Local;
use clap::Parser;
use cli::Cli;
use indicatif::ProgressBar;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use tera::{Context, Tera};

use email::{create_email, Output, Settings};
use recipients::Recipients;

trait FromHashMap {
    fn from_hashmap(map: HashMap<String, String>) -> Self;
}

impl FromHashMap for Context {
    fn from_hashmap(map: HashMap<String, String>) -> Self {
        let mut context = Context::new();
        for (key, value) in map {
            context.insert(&key, &value);
        }
        context
    }
}

fn process_recipient(
    recipient: &mut recipients::Recipient,
    subject: &str,
    content: &str,
) -> Result<(String, String), tera::Error> {
    let context = Context::from_hashmap(recipient.variables.clone());
    let body = Tera::one_off(&content, &context, true)?;
    let personalized_subject = Tera::one_off(&subject, &context, true)?;
    Ok((body, personalized_subject))
}

fn send(settings: &Settings, recipients: Recipients, subject: &str, content: &str) {
    let creds = Credentials::new(settings.username.clone(), settings.password.clone());
    let mailer = SmtpTransport::starttls_relay(&settings.smtp_relay)
        .unwrap()
        .credentials(creds)
        .build();

    let recipient_count = recipients.len() as u64;
    let progress_bar = ProgressBar::new(recipient_count);

    let now = Local::now();
    let datetime = now.format("%Y%m%dT%H%M%S").to_string();
    let output_filename = format!("output-{}.csv", datetime);
    let mut writer = csv::Writer::from_path(output_filename).unwrap();

    let mut sent_count = 0;

    for mut recipient in recipients {
        progress_bar.set_message(format!("{}: Sending...", &recipient.email));

        match process_recipient(&mut recipient, subject, content) {
            Ok((personalized_subject, body)) => {
                let email = create_email(
                    &settings.from,
                    &settings.reply_to,
                    &recipient.email,
                    &personalized_subject,
                    &body,
                );

                match mailer.send(&email) {
                    Ok(_) => {
                        progress_bar.set_message(format!("{}: Sent", &recipient.email));
                        recipient.sent = true;
                        sent_count += 1;
                    }
                    Err(e) => {
                        progress_bar.set_message(format!("{}: {}", &recipient.email, e));
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
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

    progress_bar.finish_with_message(format!("Sent {}/{}", sent_count, recipient_count));
    writer.flush().unwrap();
}

fn preview(recipients: Recipients, subject: &str, content: &str) {
    let recipient_count = recipients.len() as u64;

    for (i, mut recipient) in recipients.into_iter().enumerate() {
        match process_recipient(&mut recipient, subject, content) {
            Ok((body, personalized_subject)) => {
                println!("TO:   {}", recipient.email.email.to_string());
                println!("SUBJECT: {}", personalized_subject);
                println!("BODY:\n{}", body);
                println!("Previewing {} of {}", i + 1, recipient_count);
                println!("Press enter to continue...");

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let cli = Cli::parse();
    let content = fs::read_to_string(&cli.message)?;

    let reader: Box<dyn BufRead> = match &cli.recipients {
        Some(path) => Box::new(BufReader::new(fs::File::open(path).unwrap())),
        None => Box::new(BufReader::new(std::io::stdin())),
    };

    let recipients = Recipients::from_reader(reader)?;

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
            send(&settings, recipients, &cli.subject, &content);
        }
        cli::Command::Preview {} => {
            preview(recipients, &cli.subject, &content);
        }
    }
    Ok(())
}
