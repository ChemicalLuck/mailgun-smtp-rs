use std::{collections::HashMap, fs::File, path::Path};

use csv::Reader;
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
    pub fn from_path(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let csv = csv::Reader::from_path(path)?;
        Self::from_reader(csv)
    }
    pub fn from_reader(mut reader: Reader<File>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut recipients = Vec::new();
        let headers = reader.headers()?.clone();
        let mut errs = Vec::new();

        for result in reader.records() {
            let row = result?;
            let recipient: Result<Recipient, csv::Error> = row.deserialize(Some(&headers));
            match recipient {
                Ok(recipient) => recipients.push(recipient),
                Err(err) => match err.kind() {
                    csv::ErrorKind::Deserialize { pos, err } => {
                        errs.push((pos.clone(), err.clone()));
                        eprintln!("Error on line {}: {}", pos.clone().unwrap().line(), err)
                    }
                    _ => todo!(),
                },
            }
        }

        if !errs.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("{} errors", errs.len()),
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
