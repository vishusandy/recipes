use regex::Regex;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use chrono::NaiveDate;
use recipe_structs::*;
use recipe_impl::*;
use entries::*;





pub fn date_format(d: &str) -> DateFmt {
    let ymd = Regex::new("(?P<year>[0-9]{2}(?:[0-9]{2})?)[\\./-](?P<month>[0-1]?[0-9])[\\./-](?P<day>[0-3]?[0-9])").unwrap();
    let mdy = Regex::new("(?P<month>[0-1]?[0-9])[\\./-](?P<day>[0-3]?[0-9])[\\./-](?P<year>[0-9]{2}(?:[0-9]{2})?)").unwrap();
    
    match d {
        x if ymd.is_match(x) => DateFmt::Ymd,
        x if mdy.is_match(x) => DateFmt::Mdy,
        _ => DateFmt::None,
    }
    
}

pub fn config() -> RecipeConfig {
    let mut nr: u32  = 0;
    let mut nc: u32  = 0;
    let mut air: u32 = 0;
    let mut aic: u32 = 0;
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
    }
}
// pub fn cfg<'a>(c: &'a mut RecipeConfig) -> &'a mut RecipeConfig {
    // let mut rcfg: RecipeConfig = helpers::config();

pub fn cfg() -> RecipeConfig {
    let mut rcfg: RecipeConfig = config();
    rcfg
}

pub fn show_config(rcfg: &RecipeConfig) {
    
    println!("Config:\nnumcontribs = {}\nnumrecipes = {}\nnextrid = {}\nnextcid = {}", rcfg.num_contribs, rcfg.num_recipes, rcfg.ai_rid, rcfg.ai_cid);

}
