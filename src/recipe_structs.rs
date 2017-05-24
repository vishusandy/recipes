use chrono::NaiveDate;

pub struct Recipe {
    pub rid: u32,
    pub title: String,
    pub date: NaiveDate, //date
    pub contributor: Contrib,
    pub ingredients: String,
    pub directions: String,
}

pub struct Contrib {
    pub cid: u32,
    pub added: NaiveDate, //date
    pub name: String,
    pub city: String,
    pub state: String,
    pub comments: String,
    
}

pub struct RecipeConfig {
    pub num_recipes: u32,
    pub num_contribs: u32,
    pub ai_rid: u32,
    pub ai_cid: u32,
}

pub enum DateFmt {
    Ymd,
    Mdy,
    None,
}
