use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Date {
    year: i16,
    month: i8,
    day: i8,
}

impl Date {
    fn new(input: &str) -> Date {
        let mut parts = input.splitn(3, '-');
        let year = parts.next().unwrap().parse().unwrap();
        let month = parts.next().unwrap().parse().unwrap();
        let day = parts.next().unwrap().parse().unwrap();
        
        Date { year, month, day }
    }
}

// Implement custom deserialization for Date
impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateVisitor;

        impl<'de> Visitor<'de> for DateVisitor {
            type Value = Date;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a date string in the format YYYY-MM-DD")
            }

            fn visit_str<E>(self, value: &str) -> Result<Date, E>
            where
                E: de::Error,
            {
                Ok(Date::new(value))
            }
        }

        deserializer.deserialize_str(DateVisitor)
    }
}

// Custom deserialization function for f64 fields
fn deserialize_f64_from_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(de::Error::custom)
}

#[derive(Debug, Deserialize)]
struct DebtRecord {
    record_date: Date,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    debt_held_public_amt: f64,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    intragov_hold_amt: f64,
    #[serde(deserialize_with = "deserialize_f64_from_string")]
    tot_pub_debt_out_amt: f64,
}

fn read_file_to_string(p: &str) -> std::io::Result<String> {
    let mut file = File::open(p)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

struct json_body {
    //{data:[DebtRecord, DebtRecord, ...]}
    data: Vec<DebtRecord>
}



fn main() {
    let list_of_debt_records = read_file_to_string("debt_records.json").unwrap();
    // Vec<DebtRecord>
    let debt_records: json_body = Vec::from(list_of_debt_records);

    let json_data = r#"
    {
        "record_date": "2023-11-28",
        "debt_held_public_amt": "26777703430303.32",
        "intragov_hold_amt": "7074148512644.33",
        "tot_pub_debt_out_amt": "33851851942947.65"
    }
    "#;

    let debt_record: DebtRecord = serde_json::from_str(json_data).unwrap();
    println!("{:?}", debt_record);
}