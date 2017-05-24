
use recipe_structs::*;
use entries::*;
use helpers::*;
use chrono::NaiveDate;


pub trait AutoInc {
    fn next() -> u32;
    fn preview() -> u32;
    fn inc() -> u32;
}

impl AutoInc for RecipeIdx {
    fn next() -> u32 {
        0
    }
    fn preview() -> u32 {
        0
    }
    fn inc() -> u32 {
        0
    }
}

impl AutoInc for ContribIdx {
    fn next() -> u32 {
        0
    }
    fn preview() -> u32 {
        0
    }
    fn inc() -> u32 {
        0
    }
}

