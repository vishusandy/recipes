
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
// use rmp_serialize::{Encoder, Decoder};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
// use std::mem;
// use chrono::NaiveDate;
use recipe_structs::*;
use recipe_impl::*;
use helpers::*;

const GSP: u8 = 29;
const RSP: u8 = 30;
const USP: u8 = 31;
const SOT: u8 = 2;
const EOT: u8 = 3;

const IDX: char = '\u{0084}';
const SOS: char = '\u{0098}';
const EOS: char = '\u{009c}';



impl Recipe {    
    pub fn display(&self) {
        println!("Recipe {}\nTitle:\t{}\nDate:\t{}\nContributor:\t{}\nIngredients:\n{}\n\nDirections:\n{}\n\n", self.rid, self.title, self.date, self.contributor, self.ingredients, self.directions);
    }

    pub fn add(r: &Recipe) -> u32 {
        0
    }
    
    pub fn addrecipe(r: &Recipe) -> u32 {
        
        
        0
    }
    
    pub fn rsearch_rid() {
        
    }
    pub fn rsearch_text() {
        
    }
    pub fn rsearch_contrib() {
        
    }
    
    pub fn readrecipe(id: u32) -> Recipe {
        let rid: u32 = 0;
        let title = String::new();
        let date = String::new(); // NaiveDate = NaiveDate::from_ymd(2017, 5, 24);
        let contributor: Contrib = Contrib {
            cid: 0,
            added: String::new(), // NaiveDate::from_ymd(2017, 5, 24),
            name: String::new(),
            city: String::new(),
            state: String::new(),
            comments: String::new(),
        }; 
        let contributor: u32 = 0;
        let ingredients = String::new();
        let directions = String::new();
        
        Recipe {
            rid,
            title,
            date,
            contributor,
            ingredients,
            directions,
        }
    }

    pub fn writerecipe(item: &Recipe) -> bool {
        let mut success = false;
        
        let mut buf = Vec::new();
        item.display();
        
        item.serialize(&mut Serializer::new(&mut buf)).unwrap();
        println!("Serialize Item buffer: \n{:?}\n", buf);
        
        success
    }
    
    //write all recipes in a collection to file
    pub fn writerecipes(list: &Vec<Recipe>) -> bool {
        let mut success = false;
        let mut fs = File::create("recipes.db");
        
        // let mut f = BufWriter::new(fs.expect("Could not open file"));
        let mut f = fs.expect("Could not open file");
        let mut buf = Vec::new();
        list.serialize(&mut Serializer::new(&mut buf)).expect("Could not serialze recipes");
        println!("Serialize Item buffer: \n{:?}\n", buf);
        
        f.write(&buf);
        // f.sync_all();
        success
    }

    pub fn readrecipes(list: &mut Vec<Recipe>) -> bool {
        let mut success = false;
        
        /*
        let mut buf = Vec::new();
        recipelist.serialize(&mut Serializer::new(&mut buf)).unwrap();
        println!("Serialize Item buffer: \n{:?}\n", buf);
        
        let mut ds = Deserializer::new(&buf[..]);
        let mut uns: Vec<Recipe> = Deserialize::deserialize(&mut ds).unwrap();
        for item in uns {
            item.display();
        }
        */


        // let mut f = BufReader::new(File::open("recipes.db").expect("Could not open recipes database"));
        let mut f = File::open("recipes.db").expect("Could not open recipes database");
        
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer);
        println!("\nFile Contents:\n{:?}\n", buffer);
        let mut ds = Deserializer::new(&buffer[..]);
        // println!("\nDeserialize Buffer:\n{:?}\n", ds);
        *list = Deserialize::deserialize(&mut ds).expect("Could not deserialize recipe data");
        // list = Deserialize::deserialize(&mut ds).expect("Could not deserialize recipe data");
        
        
        /*
        // let mut bufstr = String::new();
        // f.read_to_string(&mut bufstr);
        // println!("\nFile Contents:\n{:?}\n", bufstr);
        let mut bufstr = String::new();
        f.read_to_end(&mut bufstr);
        println!("\nFile Contents:\n{:?}\n", bufstr);
        let buf = bufstr.as_bytes().to_vec();
        println!("\nFile Contents Buffer:\n{:?}\n", buf);
        let mut ds = Deserializer::new(&buf[..]);
        // println!("\nDeserialize Buffer:\n{:?}\n", ds);
        list.clear();
        
        let mut bufstr = String::new();
        f.read_to_string(&mut bufstr);
        println!("\nFile Contents:\n{:?}\n", bufstr);
        let buf = bufstr.as_bytes().to_vec();
        println!("\nFile Contents Buffer:\n{:?}\n", buf);
        let mut ds = Deserializer::new(&buf[..]);
        
        
        // println!("\nDeserialize Buffer:\n{:?}\n", ds);
        
        list.clear();
        */
        
        // let mut list2: Vec<Recipe> = Deserialize::deserialize(&mut ds).expect("Could not deserialize recipe data");
        
        // let mut uns: Vec<Recipe> = Deserialize::deserialize(&mut ds).unwrap();
        /* list.clear();
        for item in uns {
            list.push(item);
            item.display();
        } */
        
        
        success
    }

    pub fn allrecipes(v: &mut Vec<Recipe>) -> bool {
        //todo add check if not exist then create file
        let mut f = BufReader::new(File::open("recipes.db").unwrap());
        let mut b = String::new();
        let mut success = false;
        f.read_to_string(&mut b).expect("Could not open recipe data.");
        v.clear();
        let mut buf = Vec::new();
        let mut ds = Deserializer::new(&buf[..]);
        let mut uns: Recipe = Deserialize::deserialize(&mut ds).unwrap();
        uns.display();
        
        success
    }
}




