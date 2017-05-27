

use CFG;
use recipe_structs::*;
use entries::*;
use helpers::*;
use chrono::NaiveDate;


impl RecipeConfig {
    pub fn nextrid() -> u32 {
        unsafe {
            let t = (*CFG).ai_rid;
            // (*CFG).ai_rid += 1;
            t
        }
    }

    pub fn previewrid() -> u32 {
        unsafe {
            (*CFG).ai_rid
        }
    }
    pub fn incrid() -> u32 {
        unsafe {
            // (*CFG).ai_rid += 1;
            return (*CFG).ai_rid
        }
    }

    pub fn nextrecipe(&mut self) -> u32 { // returns the next available rid from the config
        let t = self.ai_rid;
        self.ai_rid += 1;
        t
    } 
    pub fn previewrecipe(&self) -> u32 {
        self.ai_rid
    }
    pub fn increcipe(&mut self) -> u32 { // returns the rid + 1 (the rid after incremented)
        self.ai_rid += 1;
        self.ai_rid
    }
}

pub trait AutoInc {
    fn next(&mut RecipeConfig) -> RecipeIdx;
    fn preview(&RecipeConfig) -> u32;
    fn inc(&mut RecipeConfig) -> u32;
    fn add(&mut RecipeConfig, u32) -> u32;
}

impl AutoInc for RecipeIdx {
    fn next(c: &mut RecipeConfig) -> RecipeIdx {
        RecipeIdx::Index(Self::inc(c)-1)
        // RecipeIdx::Index(c.ai_rid)
    }
    fn preview(c: &RecipeConfig) -> u32 {
        c.ai_rid
    }
    fn inc(c: &mut RecipeConfig) -> u32 {
        c.ai_rid += 1;
        c.ai_rid
    }
    fn add(c: &mut RecipeConfig, n: u32) -> u32 {
        c.ai_rid += n;
        c.ai_rid
    }
}

impl AutoInc for ContribIdx {
    fn next(c: &mut RecipeConfig) -> RecipeIdx {
        RecipeIdx::Index(0)
    }
    fn preview(c: &RecipeConfig) -> u32 {
        c.ai_cid
    }
    fn inc(c: &mut RecipeConfig) -> u32 {
        c.ai_cid += 1;
        c.ai_cid
    }
    fn add(c: &mut RecipeConfig, n: u32) -> u32 {
        c.ai_cid += n;
        c.ai_cid
    }
}

