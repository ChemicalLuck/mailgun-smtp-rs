use std::{collections::HashMap, io::BufRead};

use lettre::message::Mailbox;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Recipient {
    pub email: Mailbox,
    #[serde(flatten)]
    pub variables: HashMap<String, String>,
    #[serde(default)]
    pub sent: bool,
}

pub struct Recipients {
    records: Vec<Recipient>,
}

impl From<Vec<Recipient>> for Recipients {
    fn from(records: Vec<Recipient>) -> Self {
        Self { records }
    }
}

impl Recipients {
    pub fn from_reader(reader: Box<dyn BufRead>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut csv = csv::Reader::from_reader(reader);
        let headers = csv.headers()?.clone();
        let mut recipients = Vec::new();
        let mut errors = Vec::new();

        for (i, result) in csv.records().enumerate() {
            match result {
                Ok(record) => match record.deserialize(Some(&headers)) {
                    Ok(recipient) => recipients.push(recipient),
                    Err(err) => {
                        errors.push(format!("Error on record {}: {}", i + 1, err.to_string()));
                    }
                },
                Err(err) => {
                    errors.push(format!(
                        "CSV reading error on record {}: {}",
                        i + 1,
                        err.to_string()
                    ));
                }
            }
        }

        if !errors.is_empty() {
            eprintln!("{}", errors.join("\n"));
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{} errors", errors.len()),
            )
            .into());
        }

        Ok(Self {
            records: recipients,
        })
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
}

impl IntoIterator for Recipients {
    type Item = Recipient;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.records.into_iter()
    }
}
