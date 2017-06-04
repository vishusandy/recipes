/*
recipes
    title date contributor ingredients directions
contributors
    name city state comments
*/

extern crate regex;
extern crate chrono;
extern crate time;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate rmp_serde as rmps;
// extern crate rmp_serialize;

// use serde::ser::{Serialize, Serializer, SerializeStruct};


mod helpers;
mod entries;
mod people;
mod recipe_structs;
mod autoinc;
mod tags;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::mem;
use std::collections::{HashMap, HashSet};
use std::time::{Instant, Duration};

use autoinc::*;
use helpers::*;
use recipe_structs::*;
use entries::*;
use people::*;
use tags::*;

static mut CFG: *const RecipeConfig = 0 as *const RecipeConfig;
static mut RECIPELIST: *const Vec<Recipe> = 0 as *const Vec<Recipe>;
static mut RECIPEDICT: *const HashMap<u32, &mut Recipe> = 0 as *const HashMap<u32, &mut Recipe>;
static mut CONTRIBLIST: *const Vec<Contrib> = 0 as *const Vec<Contrib>;
static mut CONTRIBDICT: *const HashMap<u32, &mut Contrib> = 0 as *const HashMap<u32, &mut Contrib>;
static mut ALLTAGS: *const HashMap<String, u16> = 0 as *const HashMap<String, u16>;
static mut ALLTIDS: *const HashMap<u16, String> = 0 as *const HashMap<u16, String>;

fn main() {
    let beginning = Instant::now();
    // let mut rcfg: RecipeConfig = RecipeConfig::config();
    let mut rcfg: RecipeConfig = RecipeConfig::new();
    let mut rlist: Vec<Recipe> = Vec::new();
    let mut rdict: HashMap<u32, &mut Recipe> = HashMap::new();
    let mut clist: Vec<Contrib> = Vec::new();
    let mut cdict: HashMap<u32, &mut Contrib> = HashMap::new();
    let mut atags: HashMap<String, u16> = HashMap::new();
    let mut atids: HashMap<u16, String> = HashMap::new();
    
    unsafe {
        CFG = mem::transmute(&rcfg);
        RECIPELIST = mem::transmute(&rlist);
        RECIPEDICT = mem::transmute(&rdict);
        CONTRIBLIST = mem::transmute(&clist);
        CONTRIBDICT = mem::transmute(&cdict);
        ALLTAGS = mem::transmute(&atags);
        ALLTIDS = mem::transmute(&atids);
    }
    
    // show_config();
    
    let mut recipelist: Vec<Recipe> = vec![
        Recipe {
            rid: 0u32,
            title: String::from("Lasagna"),
            // date: String::from("2017-05-26"),
            date: NaiveDate::from_ymd(2017, 5, 31),
            contributor: 1u32,
            ingredients: String::from("Lasagna Noodles\nTomatoes\nCheeses"),
            directions: String::from("1. Cook noodles\n2. Add ingredients\n3. Cook other stuff\n4. More and really long and complicated stuff that requires lots of cooking and patience and requires a cooking level of 99\n5. This is the final step."),
            tags: Recipe::add_tags(&vec!["italian food".to_string(), "noodles".to_string(), "sauce".to_string()]),
        },
        Recipe {
            rid: 0u32,
            title: String::from("Pizza"),
            // date: String::from("2017-05-25"),
            date: NaiveDate::from_ymd(2017, 5, 31),
            contributor: 0u32,
            ingredients: String::from("Pizza Dough\n\tFlour\n\tSemolina Flour\n\tEggs\n\tWater\n\tOil\nTomatoes\nCheese"), 
            directions: String::from("1. Add semolina flour and flour and eggs and oil\n2. Make mixture into dough\n3. Make tomatoes into sauce\n4. Add sauce and cheese and any toppings to the pizza\n5. Bake in oven at ow my hand tempatures\n6. Die from happiness"),
            tags: Recipe::add_tags(&vec!["za".to_string(), "italian".to_string(), "italian food".to_string(), "college food".to_string(), "customizable".to_string(), "best food ever".to_string()]),
        },
    ];
    
    let mut peoples: Vec<Contrib> = vec![
        Contrib {
            cid: 0u32,
            // added: String::from("2017-05-28"),
            added: NaiveDate::from_ymd(2017, 5, 31),
            name: String::from("Andrew Prindle"),
            city: String::from("Madison"),
            state: String::from("Wi"),
            comments: String::from("The best person ever in the world"),
        },
        Contrib {
            cid: 0u32,
            // added: String::from("2017-05-28"),
            added: NaiveDate::from_ymd(2018, 5, 31),
            name: String::from("Jason Smith"),
            city: String::from("Madison"),
            state: String::from("Wi"),
            comments: String::from("The most awesomest neighbor, super nicest person you;ll ever meet"),
        },
        Contrib {
            cid: 0u32,
            // added: String::from("2017-05-28"),
            added: NaiveDate::from_ymd(2019, 5, 31),
            name: String::from("Kelly"),
            city: String::from("Madison"),
            state: String::from("Wi"),
            comments: String::from("She has an awesome dog named Rosebud and a really an adorable son named Owen."),
        }
    ];
    let c1 = peoples[0].add();
    let c2 = peoples[1].add();
    let c3 = peoples[2].add();
    
    
    Contrib::writecontribs();
    Contrib::readcontribs();
    
    recipelist[0].add();
    recipelist[1].add();
    println!("Creating recipe #{}", Recipe::create(
        0, String::from("Bread"), String::from(""), c2, 
        String::from("Flour\nEggs\nYeast\nStuff\nA UFO\nGeorge Clooney"),
        String::from("1. Ground flour into flour\n2. Beat eggs into a pulp\n3. Add yeast into flour\n4. Add eggs into mixture\n5. Remove flour from mixture\n6. Throw away mixture\n7. Use UFO to magically make bread."),
        String::from("pizzA,dough,eGgs,itAliAn FOOd,College FoOd")
    ));
    
    Recipe::writetags();
    Recipe::readtags();
    
    println!("Showing tags");
    Recipe::print_tags();
    // Recipe::print_tids();
    
    Recipe::writerecipes();
    Recipe::readrecipes();
    
    // Contrib::writecontribs();
    // Contrib::readcontribs();
    // show_config();

    // Display recipes
    println!("Showing Recipes:");
    for (_, rec) in &rdict { rec.display(); }
    
    // Display contributors
    // println!("Showing Contributors");
    // for (_, ctb) in &cdict { ctb.display(); }
    
    show_config();
    // RecipeConfig::write();
    
    // Search Recipes for text string
    let searchstr: String = "itAlIAn fOOD".to_string();
    println!("\n----SEARCHING----------------\nSearching for {}", searchstr);
    let rsts: Vec<&Recipe> = Recipe::search_text(&searchstr);
    // println!("Search results raw:\n{:#?}", rsts);
    for item in &rsts { item.display(); }
    

    // println!("\nShowing Rlist:\n{:#?}", rlist);
    // println!("\nShowing Rdict:\n{:#?}", rdict);
    
    let ending = beginning.elapsed();
    println!("Exec time: {}.{:08}", ending.as_secs(), ending.subsec_nanos());
    
}
