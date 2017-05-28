
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::mem;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::collections::HashMap;

use recipe_structs::*;
use autoinc::*;
use helpers::*;
use entries::*;

use {CFG, RECIPELIST, RECIPEDICT, CONTRIBLIST, CONTRIBDICT};

impl Contrib {
    pub fn display(&self) {
        println!("Contributor {}\nName: {}\nAdded on: {}\nLocation: {}, {}\nComments:\n{}\n\n", self.cid, self.name, self.added, self.city, self.state, self.comments);
    }
    
    pub fn add(&self) -> u32 {
        let mut clist: &mut Vec<Contrib>;
        let mut cdict: &mut HashMap<u32, &mut Contrib>;
        unsafe {
            clist = mem::transmute(CONTRIBLIST);
            cdict = mem::transmute(CONTRIBDICT);
        }
        
        let cid = match self.cid {
            0 => RecipeConfig::nextcid(),
            x => x,
        };
        clist.push(Contrib {
           cid: cid, added: self.added.to_owned(), name: self.name.to_owned(), city: self.city.to_owned(), state: self.state.to_owned(), comments: self.comments.to_owned()
        });
        cdict.insert(cid, clist.last_mut().unwrap());
        cid
    }
    
    pub fn add_contrib(c: &Contrib) -> u32 {
        
        0
    }
    
    pub fn new(id: u32, added: String, name: String, city: String, state: String, comments: String) -> u32 {
        
        0
    }
    
    pub fn search_cid<'a>(id: u32) -> ResultC<'a> {
        let mut cdict: &mut HashMap<u32, &mut Contrib>;
        unsafe {
            cdict = mem::transmute(CONTRIBDICT);
        }
        match cdict.get(&id) {
            None => ResultC::Fail("Person id not found"),
            Some(c) => ResultC::Result(&c),
        }
    }
    
    pub fn search_text<'a>(textin: &str) -> Vec<&'a Contrib> {
        let textlower = textin.to_lowercase();
        let text = textlower.as_str();
        let mut results: Vec<&Contrib> = Vec::new();
        let clist: &Vec<Contrib>;
        unsafe {
            clist = mem::transmute(CONTRIBLIST);
        }
        for item in clist {
            if item.name.to_lowercase().contains(text) || item.city.to_lowercase().contains(text) || item.state.to_lowercase().contains(text) || item.comments.to_lowercase().contains(text) {
            // if item.name.to_lowercase().contains(text) || item.city.to_lowercase().contains(text) || item.state.to_lowercase().contains(text) || item.comments.to_lowercase().contains(text) {
                results.push(item);
            }
        }
        results
    }
    
    pub fn writecontribs() -> bool {
        let mut success = true;
        let mut clist: &mut Vec<Contrib>;
        let mut cdict: &mut HashMap<u32, &mut Contrib>;
        let mut cfg: &mut RecipeConfig;
        unsafe {
            cfg = mem::transmute(CFG);
            clist = mem::transmute(CONTRIBLIST);
            cdict = mem::transmute(CONTRIBDICT);
        }
        let mut fs = File::create("contribs.db");
        let mut f = BufWriter::new(fs.expect("Could not read contrib database"));
        let mut buf = Vec::new();
        clist.serialize(&mut Serializer::new(&mut buf)).expect("Could not serialize contributors");
        f.write(&buf);
        cfg.num_contribs = clist.len() as u32;
        
        success
    }
    
    pub fn readcontribs() -> bool {
        let mut success = true;
        let mut clist: &mut Vec<Contrib>;
        let mut cdict: &mut HashMap<u32, &mut Contrib>;
        let mut cfg: &mut RecipeConfig;
        unsafe {
            clist = mem::transmute(CONTRIBLIST);
            cdict = mem::transmute(CONTRIBDICT);
            cfg = mem::transmute(CFG);
        }
        let mut f = File::open("contribs.db").expect("Could not open contrib database");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);
        let mut de = Deserializer::new(&buffer[..]);
        *clist = Deserialize::deserialize(&mut de).expect("Could not deserialize contributor data");
        cfg.num_contribs = clist.len() as u32;
        
        cdict.clear();
        let mut maxcid = 0;
        for item in clist {
            if item.cid > maxcid {
                maxcid = item.cid;
            }
            cdict.insert(item.cid, item);
        }
        cfg.ai_cid = maxcid + 1;
        cfg.num_contribs = cdict.len() as u32;
        println!("Max cid: {}", maxcid);
        println!("Number of contributors: {}", cdict.len());
        
        success
    }
    
}


















