use chrono::Date;

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub rid: u32, //todo: convert to `type RecipeIdx = u32` or enum RecipeIdx instead of u32
    pub title: String,
    pub date: String, //change to NaiveDate
    pub contributor: u32, //todo: change to &Contrib
    pub ingredients: String,
    pub directions: String,
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
}

pub enum DateFmt<'a> {
    // Ymd(u16, u8, u8),
    // Mdy(u8, u8, u16),
    Ymd(u32, u32, u32),
    Mdy(u32, u32, u32),
    Fail(&'a str),
    None,
}
