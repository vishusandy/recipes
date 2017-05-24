use regex::Regex;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use recipe_structs::*;

pub fn date_format(d: &str) -> DateFmt {
    let ymd = Regex::new("(?P<year>[0-9]{2}(?:[0-9]{2})?)[\\./-](?P<month>[0-1]?[0-9])[\\./-](?P<day>[0-3]?[0-9])").unwrap();
    let mdy = Regex::new("(?P<month>[0-1]?[0-9])[\\./-](?P<day>[0-3]?[0-9])[\\./-](?P<year>[0-9]{2}(?:[0-9]{2})?)").unwrap();
    
    match d {
        x if ymd.is_match(x) => DateFmt::Ymd,
        x if mdy.is_match(x) => DateFmt::Mdy,
        _ => DateFmt::None,
    }
    
}

// specify the type using the syntax `<recipe_structs::Cfg as Trait>::RecipeConfg`
pub fn config() -> RecipeConfig {
    let mut nr: u32 = 0;
    let mut nc: u32 = 0;
    let mut air: u32 = 0;
    let mut aic: u32 = 0;
    let f = BufReader::new(File::open("recipes.cfg").unwrap());
    
    for line in f.lines() {
        match line {
            Err(e) => println!("Error: {}", e),
            Ok(l)  => {
                // let t: Vec<&str> = match l {
                match l {
                    // v if v.starts_with("numcontribs = ") => nc = v.split_terminator('=').collect::<Vec<&str>>().last().unwrap().trim().parse::<u32>().expect("numcontribs must be a number"),
                    ref v if v.starts_with("numcontribs = ") => nc = v.split_terminator('=').collect::<Vec<&str>>().last().unwrap().trim().parse::<u32>().expect("numcontribs must be a number"),
                    ref v if v.starts_with("numrecipes = ") => nr = v.split_terminator('=').collect::<Vec<&str>>().last().unwrap().trim().parse::<u32>().expect("numrcipes must be a number"),
                    ref v if v.starts_with("nextrid = ") => air = v.split_terminator('=').collect::<Vec<&str>>().last().unwrap().trim().parse::<u32>().expect("nextrid must be a number"),
                    ref v if v.starts_with("nextcid = ") => aic = v.split_terminator('=').collect::<Vec<&str>>().last().unwrap().trim().parse::<u32>().expect("nextcid must be a number"),
                    // v if v.starts_with("numrecipes = ") => nr = v.split_terminator('=').collect::<Vec<&str>>().last().trim().parse::<i32>(),
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

