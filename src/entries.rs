
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
// use rmp_serialize::{Encoder, Decoder};
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

use {CFG, RECIPELIST, RECIPEDICT, CONTRIBLIST, CONTRIBDICT, ALLTAGS};

impl Recipe {    
    pub fn display(&self) {
        //calculate size of all tags and create a string buffer with that capacity
        
        println!("Recipe {}\nTitle:\t{}\nDate:\t{}\nContributor:\t{}\nIngredients:\n{}\n\nDirections:\n{}\n\n", self.rid, self.title, self.date, self.contributor, self.ingredients, self.directions);
    }
    
    pub fn add_tags(tags: &Vec<String>) -> Vec<u16> {
        let mut atags: &mut HashMap<String, u16>;
        let mut results: Vec<u16> = Vec::new();
        unsafe {
            atags = mem::transmute(ALLTAGS);
        }
        for tag in tags {
            results.push(
                *atags.entry(tag.to_owned()).or_insert(RecipeConfig::nexttid())
            );
        }
        results
        //vec![0u16, 5u16, 3u16, 4u16, 2u16]
    }
    
    pub fn writetags() {
        let mut atags: &mut HashMap<String, u32>;
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
        let maxtid: u16 = match atags.iter().max() {
            Some(a) => *a.1,
            None => 1u16,
        };
        cfg.ai_tid = maxtid;
    }
    
    pub fn remove_tags() {}
    
    pub fn edit_tag() {}
    
    pub fn add(&self) -> u32 {
        let mut list: &mut Vec<Recipe>; 
        let mut rdict: &mut HashMap<u32, &mut Recipe>;
        unsafe {
            list = mem::transmute(RECIPELIST);
            rdict = mem::transmute(RECIPEDICT);
        }
        
        let rid = match self.rid {
            0 => RecipeConfig::nextrid(),
            x => x,
        };
        // list.push(Recipe {
        list.push(Recipe {
            rid: rid, title: self.title.to_owned(), date: self.date.to_owned(), contributor: self.contributor, ingredients: self.ingredients.to_owned(), directions: self.directions.to_owned(), tags: self.tags.to_owned(),
        });
        rdict.insert(rid, list.last_mut().unwrap());
        rid
    }

    
    pub fn search_rid<'a>(id: u32) -> ResultR<'a> {
        
        let mut rdict: &mut HashMap<u32, &mut Recipe>;
        unsafe {
            rdict = mem::transmute(RECIPEDICT);
        }
        match rdict.get(&id) {
            None => ResultR::Fail("Entry not found"),
            Some(r) => ResultR::Result(&r),
        }
        /*
        match rdict.contains_key(&id) {
            false => ResultR::Fail("Entry not found"),
            true  => ResultR::Result(rdict.get(&id)),
        }*/
    }
    pub fn search_text<'a>(textin: &str) -> Vec<&'a Recipe> {
        let textlower = textin.to_lowercase();
        let text = textlower.as_str();
        let mut results: Vec<&Recipe> = Vec::new();
        let list: &Vec<Recipe>;
        unsafe {
            list = mem::transmute(RECIPELIST);
        }
        for item in list {
            // if item + 5 == 0 {}
            // todo: match against date and possibly maybe contributor information?
            // todo: search for text in each of the tags associated with the recipe
            if item.title.to_lowercase().contains(text) || item.ingredients.to_lowercase().contains(text) || item.directions.to_lowercase().contains(text) {
                results.push(item);
            }
        }
        results
    }
    
    //write all recipes in a collection to file
    pub fn writerecipes() -> bool {
        let mut success = true;
        let mut list: &mut Vec<Recipe>;
        let mut rdict: &mut HashMap<u32, &mut Recipe>;
        let mut m: &mut RecipeConfig;
        unsafe {
            m = mem::transmute(CFG);
            list = mem::transmute(RECIPELIST);
            rdict = mem::transmute(RECIPEDICT);
        }
        let mut fs = File::create("recipes.db");
        
        let mut f = BufWriter::new(fs.expect("Could not read recipe database"));
        let mut buf = Vec::new();
        list.serialize(&mut Serializer::new(&mut buf)).expect("Could not serialze recipes");
        // println!("Serialize Item buffer: \n{:?}\n", buf);
        
        f.write(&buf);
        
        m.num_recipes = list.len() as u32;
        
        // c.num_recipes = list.len() as u32;
        // f.sync_all();
        success
    }

    pub fn readrecipes() -> bool {
        let mut success = true;
        let mut list: &mut Vec<Recipe>;
        let mut rdict: &mut HashMap<u32, &mut Recipe>;
        let mut cfg: &mut RecipeConfig;
        unsafe {
            list = mem::transmute(RECIPELIST);
            rdict = mem::transmute(RECIPEDICT);
            cfg = mem::transmute(CFG);
        }
        
        let mut f = File::open("recipes.db").expect("Could not open recipes database");
        
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);
        // println!("\nFile Contents:\n{:?}\n", buffer);
        let mut ds = Deserializer::new(&buffer[..]);
        *list = Deserialize::deserialize(&mut ds).expect("Could not deserialize recipe data");
        cfg.num_recipes = list.len() as u32;
        
        rdict.clear();
        let mut maxrid = 0;
        for item in list { // item = &mut Recipe
            // println!("\nReading:\n{:?}", item);
            if item.rid > maxrid {
                maxrid = item.rid;
            }
            rdict.insert(item.rid, item);
        }
        // let maxrid = list.iter().max().unwrap_or(0) + 1;
        
        cfg.ai_rid = maxrid + 1;
        cfg.num_recipes = rdict.len() as u32;
        println!("Max rid: {}", maxrid);
        println!("Number of recipes: {}", rdict.len());
        
        success
    }
    
    pub fn add_recipe(r: &Recipe) -> u32 {
        // let (rid, title, date, contributor, ingredients, directions) = r.tuple();
        let mut list: &mut Vec<Recipe>;
        let mut rdict: &mut HashMap<u32, &mut Recipe>;
        unsafe {
            list = mem::transmute(RECIPELIST);
            rdict = mem::transmute(RECIPEDICT);
        }
        let rid = match r.rid {
            0 => RecipeConfig::nextrid(),
            x => x,
        };
        list.push(Recipe {
            rid: rid, title: r.title.to_owned(), date: r.date.to_owned(), contributor: r.contributor, ingredients: r.ingredients.to_owned(), directions: r.directions.to_owned(), tags: r.tags.to_owned(),
            // rid, title, date, contributor, ingredients, directions
        });
        rdict.insert(rid, list.last_mut().unwrap());
        rid
    }
    
    pub fn new(id: u32, title: String, date: String, contributor: u32, ingredients: String, directions: String, tags: &Vec<u16>) -> u32 {
        let mut list: &mut Vec<Recipe>;
        let mut rdict: &mut HashMap<u32, &mut Recipe>;
        unsafe {
            list = mem::transmute(RECIPELIST);
            rdict = mem::transmute(RECIPEDICT);
        }
        let rid = match id {
            0 => RecipeConfig::nextrid(),
            x => x,
        };
        list.push(Recipe {
            rid, title, date, contributor, ingredients, directions, tags: tags.to_owned(),
            // rid: self.rid, title: self.title.to_owned(), date: self.date.to_owned(), contributor: self.contributor, ingredients: self.ingredients.to_owned(), directions: self.directions.to_owned()
        });
        rdict.insert(rid, list.last_mut().unwrap());
        rid
    }
    
    pub fn tuple(&self) -> (&u32, &String, &String, &u32, &String, &String, &Vec<u16>) {
        let rid = &self.rid;
        let title = &self.title;
        let date = &self.date;
        let contributor = &self.contributor;
        let ingredients = &self.ingredients;
        let directions = &self.directions;
        let tags = &self.tags;
        (rid, title, date, contributor, ingredients, directions, tags)
    }
    
    
}




