
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::mem;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::collections::{HashMap, HashSet};
// use chrono::NaiveDate;
use recipe_structs::*;
use autoinc::*;
use helpers::*;
use people::*;

use {CFG, RECIPELIST, RECIPEDICT, CONTRIBLIST, CONTRIBDICT, ALLTAGS, ALLTIDS};

impl Recipe {
    
    
    pub fn create_tag() {}
    pub fn change_tag() {}
    pub fn delete_tag() {}
    
    pub fn print_tags() {
        let mut atags: &mut HashMap<String, u16>;
        unsafe {
            atags = mem::transmute(ALLTAGS);
        }
        print!("Printing tags: ");
        for (tag, tid) in atags {
            print!("{}:'{}', ", tid, tag);
        }
        print!("\n");
    }
    
    pub fn print_tids() {
        let mut atids: &mut HashMap<u16, String>;
        unsafe {
            atids = mem::transmute(ALLTIDS);
        }
        print!("Printing tids: ");
        for (tid, tag) in atids {
            print!("{}:'{}', ", tid, tag);
        }
        print!("\n");
    }
    
    pub fn add_tags(tags: &Vec<String>) -> Vec<u16> {
        let mut atags: &mut HashMap<String, u16>;
        let mut results: Vec<u16> = Vec::new();
        unsafe {
            atags = mem::transmute(ALLTAGS);
        }
        for t in tags {
            let mut ntid = 0u16;
            let tag = t.to_lowercase();
            if !atags.contains_key(&tag) {
                ntid = RecipeConfig::nexttid();
                // print!("Creating new tag {}\n\t", ntid);
            }
            
            let curtid = *atags.entry(tag.to_owned()).or_insert(ntid);
            // print!("Linking tag {}:''{}\n", curtid, tag);
            results.push(curtid);
            // let curtid = *atags.entry(tag.to_owned()).or_insert(RecipeConfig::nexttid());
            
        }
        results
    }
    
    pub fn maketids() {
        let mut atags: &mut HashMap<String, u16>;
        let mut atids: &mut HashMap<u16, String>;
        unsafe {
            atags = mem::transmute(ALLTAGS);
            atids = mem::transmute(ALLTIDS);
        }
        for (tag, tid) in atags {
            atids.insert(*tid, tag.to_owned());
        }
    }
    
    pub fn writetags() {
        let mut atags: &mut HashMap<String, u16>;
        let cfg: &mut RecipeConfig;
        unsafe {
            cfg = mem::transmute(CFG);
            atags = mem::transmute(ALLTAGS);
        }
        let mut fs = File::create("tags.db");
        let mut f = BufWriter::new(fs.expect("COuld not read tags database"));
        let mut buf = Vec::new();
        atags.serialize(&mut Serializer::new(&mut buf)).expect("Could not serialize tags");
        f.write(&buf);
    }
    
    pub fn readtags() {
        let mut atags: &mut HashMap<String, u16>;
        let cfg: &mut RecipeConfig;
        unsafe {
            cfg = mem::transmute(CFG);
            atags = mem::transmute(ALLTAGS);
        }
        let mut f = File::open("tags.db").expect("Could not open tags database");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);
        let mut ds = Deserializer::new(&buffer[..]);
        atags.clear();
        *atags = Deserialize::deserialize(&mut ds).expect("Could not deserialize tag data");
        
        let mut maxtid: u16 = 0u16;
        for (_, tid) in atags {
            if *tid > maxtid {
                maxtid = *tid;
            }
        }
        if maxtid <= 0u16 {
            maxtid = 1u16;
        }
        /*
        let maxtid: u16 = match atags.iter().max() {
            Some(a) => *a.1,
            None => 1u16,
        };*/
        cfg.ai_tid = maxtid + 1;
        // println!("Read tags with a max value of {}\n", maxtid);
        Recipe::maketids();
    }


}