use crate::Role::{Admin, Student, Teacher};
use clap::Parser;
use csv::{Reader, WriterBuilder};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::error::Error;

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
enum Role {
    Admin = 0,
    Teacher = 1,
    Student = 2,
}

impl Default for Role {
    fn default() -> Self {
        Student
    }
}

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    csv_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    email: String,
    username: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    displayedName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    role: Option<Role>,
}

fn random_password() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}

fn parse_csv(path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path).unwrap();
    let mut res = vec![];

    for line in rdr.deserialize() {
        let mut record: Record = line?;
        record.password = Some("1".to_string());
        res.push(record);
    }

    Ok(res)
}

fn deserialize_csv(records: Vec<Record>, path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().has_headers(true).from_path(path)?;

    for line in records {
        wtr.serialize(line)?;
    }

    Ok(())
}

fn get_default(records: &mut Vec<Record>) {
    for record in records {
        record.role = Some(Role::default());
        record.password = Some(random_password());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut records = parse_csv(&args.csv_path)?;

    get_default(&mut records);

    deserialize_csv(records, &args.csv_path)?;
    Ok(())
}
