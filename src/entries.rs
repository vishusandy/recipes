
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
// use rmp_serialize::{Encoder, Decoder};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
// use std::mem;
// use chrono::NaiveDate;
use recipe_structs::*;
use autoinc::*;
use helpers::*;

impl Recipe {    
    pub fn display(&self) {
        println!("Recipe {}\nTitle:\t{}\nDate:\t{}\nContributor:\t{}\nIngredients:\n{}\n\nDirections:\n{}\n\n", self.rid, self.title, self.date, self.contributor, self.ingredients, self.directions);
    }

    pub fn tuple(&self) -> (&u32, &String, &String, &u32, &String, &String) {
        let rid = &self.rid;
        let title = &self.title;
        let date = &self.date;
        let contributor = &self.contributor;
        let ingredients = &self.ingredients;
        let directions = &self.directions;
        (rid, title, date, contributor, ingredients, directions,)
    }

    pub fn add(&self, c: &mut RecipeConfig, list: &mut Vec<Recipe>) -> u32 {
        let rid = match self.rid {
            0 => c.nextrecipe(),
            x => x,
        };
        list.push(Recipe {
            rid: rid, title: self.title.to_owned(), date: self.date.to_owned(), contributor: self.contributor, ingredients: self.ingredients.to_owned(), directions: self.directions.to_owned()
        });
        rid
    }
    pub fn add_recipe(c: &mut RecipeConfig, list: &mut Vec<Recipe>, r: &Recipe) -> u32 {
        // let (rid, title, date, contributor, ingredients, directions) = r.tuple();
        let rid = match r.rid {
            0 => c.nextrecipe(),
            x => x,
        };
        list.push(Recipe {
            rid: rid, title: r.title.to_owned(), date: r.date.to_owned(), contributor: r.contributor, ingredients: r.ingredients.to_owned(), directions: r.directions.to_owned()
            // rid, title, date, contributor, ingredients, directions
        });
        rid
    }
    
    pub fn new(c: &mut RecipeConfig, list: &mut Vec<Recipe>, id: u32, title: String, date: String, contributor: u32, ingredients: String, directions: String) -> u32 {
        let rid = match id {
            0 => c.nextrecipe(),
            x => x,
        };
        list.push(Recipe {
            rid, title, date, contributor, ingredients, directions
            // rid: self.rid, title: self.title.to_owned(), date: self.date.to_owned(), contributor: self.contributor, ingredients: self.ingredients.to_owned(), directions: self.directions.to_owned()
        });
        
        rid
    }
    
    pub fn search_rid() {
        
    }
    pub fn search_text() {
        
    }
    pub fn search_contrib() {
        
    }
    
    //write all recipes in a collection to file
    pub fn writerecipes(c: &mut RecipeConfig, list: &Vec<Recipe>) -> bool {
        let mut success = false;
        let mut fs = File::create("recipes.db");
        
        let mut f = BufWriter::new(fs.expect("Could not open file"));
        let mut buf = Vec::new();
        list.serialize(&mut Serializer::new(&mut buf)).expect("Could not serialze recipes");
        // println!("Serialize Item buffer: \n{:?}\n", buf);
        
        f.write(&buf);
        c.num_recipes = list.len() as u32;
        // f.sync_all();
        success
    }

    pub fn readrecipes(c: &mut RecipeConfig, list: &mut Vec<Recipe>) -> bool {
        let mut success = false;
        
        let mut f = File::open("recipes.db").expect("Could not open recipes database");
        
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);
        // println!("\nFile Contents:\n{:?}\n", buffer);
        let mut ds = Deserializer::new(&buffer[..]);
        *list = Deserialize::deserialize(&mut ds).expect("Could not deserialize recipe data");
        
        c.num_recipes = list.len() as u32;
        
        success
    }

}




