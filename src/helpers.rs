use regex::Regex;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::fs::File;
use std::prelude::*;
use std::mem;

use chrono::NaiveDate;
use recipe_structs::*;
use autoinc::*;
use entries::*;
use people::*;

use {CFG, RECIPELIST, RECIPEDICT, CONTRIBLIST, CONTRIBDICT, ALLTAGS};

// static mut CFG: *const RecipeConfig = 0 as *const RecipeConfig;

pub fn date_format(d: &str) -> DateFmt {
    let ymd = Regex::new("(?P<year>[0-9]{2}(?:[0-9]{2})?)[\\./-](?P<month>[0-1]?[0-9])[\\./-](?P<day>[0-3]?[0-9])").unwrap();
    let mdy = Regex::new("(?P<month>[0-1]?[0-9])[\\./-](?P<day>[0-3]?[0-9])[\\./-](?P<year>[0-9]{2}(?:[0-9]{2})?)").unwrap();
    
    match ymd.captures(d) {
        Some(c) => DateFmt::Ymd(*&c["year"].parse::<u32>().unwrap_or(1), *&c["month"].parse::<u32>().unwrap_or(1), *&c["day"].parse::<u32>().unwrap_or(1)),
        _       => match mdy.captures(d) {
                       Some(c) => DateFmt::Ymd(*&c["year"].parse::<u32>().unwrap_or(1), *&c["month"].parse::<u32>().unwrap_or(1), *&c["day"].parse::<u32>().unwrap_or(1)),
                       _       => DateFmt::Fail("Text is not of the format: year-month-day  or  month-day-year"),
                   },
    }
}
 
pub fn show_config() {
    
    // println!("Config:\nnumcontribs = {}\nnumrecipes = {}\nnextrid = {}\nnextcid = {}\n", self.num_contribs, self.num_recipes, self.ai_rid, self.ai_cid);
    
    // println!("Config:\nnumcontribs = {}\nnumrecipes = {}\nnextrid = {}\nnextcid = {}\n", CFG.num_contribs, CFG.num_recipes, CFG.ai_rid, CFG.ai_cid);
    // println!("Config:\nnumcontribs = {}\nnumrecipes = {}\nnextrid = {}\nnextcid = {}\n", (*CFG).num_contribs, (*CFG).num_recipes, (*CFG).ai_rid, (*CFG).ai_cid);
    unsafe {
        //(*Self::CFG).ai_rid  also seems to work, at least when &self was used as a parameter
        println!("Config:\nnumcontribs = {}\nnumrecipes = {}\nnextrid = {}\nnextcid = {}\nnexttid = {}\n", (*CFG).num_contribs, (*CFG).num_recipes, (*CFG).ai_rid, (*CFG).ai_cid, (*CFG).ai_tid);
        // println!("\n\nConfig:\n{:?}\n\n", *CFG);
    }
}

impl RecipeConfig {
    pub fn new() -> RecipeConfig {
        RecipeConfig {
            num_recipes: 0u32,
            num_contribs: 0u32,
            ai_rid: 1u32,
            ai_cid: 1u32,
            ai_tid: 1u16,
        }
    }
    
    pub fn write() {
        let mut f = BufWriter::new(File::create("recipes.cfg").expect("Could not open configuration file."));
        
        unsafe {
            f.write(format!("numcontribs = {}\r\nnumrecipes = {}\r\nnextrid = {}\r\nnextcid = {}\n\nnexttid = {}\r\n",
                (*CFG).num_contribs, (*CFG).num_recipes, (*CFG).ai_rid, (*CFG).ai_cid, (*CFG).ai_tid).as_bytes());
        }
    }
    pub fn read() -> RecipeConfig {
        let mut nr: u32  = 0;
        let mut nc: u32  = 0;
        let mut air: u32 = 0;
        let mut aic: u32 = 0;
        let mut ait: u16 = 0;
        let f = BufReader::new(File::open("recipes.cfg").unwrap());
        
        for line in f.lines() {
            match line {
                Err(e) => println!("Error: {}", e),
                Ok(l)  => {
                    match l {
                        ref v if v.starts_with("numcontribs = ") => nc = v.split_terminator('=').collect::<Vec<&str>>().last().unwrap().trim().parse::<u32>().expect("numcontribs must be a number"),
                        ref v if v.starts_with("numrecipes = ")  => nr = v.split_terminator('=').collect::<Vec<&str>>().last().unwrap().trim().parse::<u32>().expect("numrcipes must be a number"),
                        ref v if v.starts_with("nextrid = ")     => air = v.split_terminator('=').collect::<Vec<&str>>().last().unwrap().trim().parse::<u32>().expect("nextrid must be a number"),
                        ref v if v.starts_with("nextcid = ")     => aic = v.split_terminator('=').collect::<Vec<&str>>().last().unwrap().trim().parse::<u32>().expect("nextcid must be a number"),
                        ref v if v.starts_with("nexttid = ")     => ait = v.split_terminator('=').collect::<Vec<&str>>().last().unwrap().trim().parse::<u16>().expect("nexttid must be a number"),
                        _ => {},
                    }
                }
            }
        }
        RecipeConfig {
                num_recipes: nr,
                num_contribs: nc,
                ai_rid: air,
                ai_cid: aic,
                ai_tid: ait,
        }
    }
    
    /* pub fn new() -> RecipeConfig {
        RecipeConfig::config()
    } */
    
}