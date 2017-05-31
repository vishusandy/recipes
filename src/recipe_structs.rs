use chrono::{DateTime, Date, NaiveDate};


#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub rid: u32, //todo: convert to `type RecipeIdx = u32` or enum RecipeIdx instead of u32
    pub title: String,
    #[serde(with = "naive_date_format")]
    pub date: NaiveDate,
    // pub date: String, //change to NaiveDate
    pub contributor: u32, //todo: change to &Contrib
    pub ingredients: String,
    pub directions: String,
    pub tags: Vec<u16>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeD {
    pub rid: u32, //todo: convert to `type RecipeIdx = u32` or enum RecipeIdx instead of u32
    pub title: String,
    // pub date: String, //change to NaiveDate
    #[serde(with = "naive_date_format")]
    pub date: NaiveDate,
    pub contributor: u32, //todo: change to &Contrib
    pub ingredients: String,
    pub directions: String,
    pub tags: Vec<u16>,
}
    
#[derive(Serialize, Deserialize, Debug)]
pub struct Contrib {
    pub cid: u32, //todo: convert to ContribIdx instead of u32
    pub added: String,
    pub name: String,
    pub city: String,
    pub state: String,
    pub comments: String,
}

pub enum ResultR<'a> {
    Result(&'a Recipe),
    Fail(&'a str), //maybe write as Fail(T) instead
}
pub enum ResultC<'a> {
    Result(&'a Contrib),
    Fail(&'a str),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeConfig {
    pub num_recipes: u32,
    pub num_contribs: u32,
    pub ai_rid: u32,
    pub ai_cid: u32,
    pub ai_tid: u16,
}

pub enum DateFmt<'a> {
    // Ymd(u16, u8, u8),
    // Mdy(u8, u8, u16),
    Ymd(u32, u32, u32),
    Mdy(u32, u32, u32),
    Fail(&'a str),
    None,
}


// use chrono::{DateTime, UTC};
// use serde::{Deserialize, Serialize};
// use rmps::{Deserializer, Serializer};

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct StructWithCustomDate {
    // DateTime supports Serde out of the box, but uses RFC3339 format. Provide
    // some custom logic to make it use our desired format.
    #[serde(with = "my_date_format")]
    pub timestamp: DateTime<UTC>,

    // Any other fields in the struct.
    // pub bidder: String,
}
*/
// mod my_date_format {
    
mod naive_date_format {
    use chrono::NaiveDate;
    // use chrono::{DateTime, UTC, TimeZone};
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

}
