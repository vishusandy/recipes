use chrono::{DateTime, UTC};
// use serde::{Deserialize, Serialize};
// use rmps::{Deserializer, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct StructWithCustomDate {
    // DateTime supports Serde out of the box, but uses RFC3339 format. Provide
    // some custom logic to make it use our desired format.
    #[serde(with = "my_date_format")]
    pub timestamp: DateTime<UTC>,

    // Any other fields in the struct.
    // pub bidder: String,
}

// mod my_date_format {
    
use chrono::{DateTime, UTC, TimeZone};
use serde::{self, Deserialize, Serializer, Deserializer};

// const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
const FORMAT: &'static str = "%Y-%m-%d";

// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer
//
// although it may also be generic over the input types T.

// pub fn serialize<S>(date: &DateTime<UTC>, serializer: S) -> Result<S::Ok, S::Error>
pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    let s = format!("{}", date.format(FORMAT)  );
    serializer.serialize_str(&s)
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<D>(D) -> Result<T, D::Error> where D: Deserializer
//
// although it may also be generic over the output types T.

// pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<UTC>, D::Error>
pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    // UTC.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    // UTC.from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    s.parse::<NaiveDate>().map_err(serde::de::Error::custom)
}

// }