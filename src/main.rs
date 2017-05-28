/*
recipes
    title date contributor ingredients directions
contributors
    name city state comments
*/

extern crate regex;
extern crate chrono;

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

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::mem;
use std::collections::HashMap;

use autoinc::*;
use helpers::*;
use recipe_structs::*;
use entries::*;
use people::*;

static mut CFG: *const RecipeConfig = 0 as *const RecipeConfig;
static mut RECIPELIST: *const Vec<Recipe> = 0 as *const Vec<Recipe>;
static mut RECIPEDICT: *const HashMap<u32, &mut Recipe> = 0 as *const HashMap<u32, &mut Recipe>;
static mut CONTRIBLIST: *const Vec<Contrib> = 0 as *const Vec<Contrib>;
static mut CONTRIBDICT: *const HashMap<u32, &mut Contrib> = 0 as *const HashMap<u32, &mut Contrib>;

fn main() {
    
    // let mut rcfg: RecipeConfig = RecipeConfig::config();
    let mut rcfg: RecipeConfig = RecipeConfig::new();
    let mut rlist: Vec<Recipe> = Vec::new();
    let mut rdict: HashMap<u32, &mut Recipe> = HashMap::new();
    let mut clist: Vec<Contrib> = Vec::new();
    let mut cdict: HashMap<u32, &mut Contrib> = HashMap::new();
    
    unsafe {
        CFG = mem::transmute(&rcfg);
        RECIPELIST = mem::transmute(&rlist);
        RECIPEDICT = mem::transmute(&rdict);
        CONTRIBLIST = mem::transmute(&clist);
        CONTRIBDICT = mem::transmute(&cdict);
    }
    
    // show_config();
    
    let mut recipelist: Vec<Recipe> = vec![
        Recipe {
            rid: 0u32,
            title: String::from("Lasagna"),
            date: String::from("2017-05-26"),
            contributor: 30u32,
            ingredients: String::from("Lasagna Noodles\nTomatoes\nCheeses"),
            directions: String::from("1. Cook noodles\n2. Add ingredients\n3. Cook other stuff\n4. More and really long and complicated stuff that requires lots of cooking and patience and requires a cooking level of 99\n5. This is the final step."),
        },
        Recipe {
            rid: 0u32,
            title: String::from("Pizza"),
            date: String::from("2017-05-25"),
            contributor: 22u32,
            ingredients: String::from("Pizza Dough\n\tFlour\n\tSemolina Flour\n\tEggs\n\tWater\n\tOil\nTomatoes\nCheese"),
            directions: String::from("1. Add semolina flour and flour and eggs and oil\n2. Make mixture into dough\n3. Make tomatoes into sauce\n4. Add sauce and cheese and any toppings to the pizza\n5. Bake in oven at ow my hand tempatures\n6. Die from happiness"),
        },
    ];
    
    let mut peoples: Vec<Contrib> = vec![
        Contrib {
            cid: 0u32,
            added: String::from("2017-05-28"),
            name: String::from("Andrew Prindle"),
            city: String::from("Madison"),
            state: String::from("Wi"),
            comments: String::from("The best person ever in the world"),
        },
        Contrib {
            cid: 0u32,
            added: String::from("2017-05-28"),
            name: String::from("Jason Smith"),
            city: String::from("Madison"),
            state: String::from("Wi"),
            comments: String::from("The most awesomest neighbor, super nicest person you;ll ever meet"),
        },
        Contrib {
            cid: 0u32,
            added: String::from("2017-05-28"),
            name: String::from("Kelly"),
            city: String::from("Madison"),
            state: String::from("Wi"),
            comments: String::from("She has an awesome dog named Rosebud and a really an adorable son named Owen."),
        }
    ];
    
    peoples[0].add();
    peoples[1].add();
    peoples[2].add();
    Contrib::writecontribs();
    Contrib::readcontribs();
    
    recipelist[0].add();
    recipelist[1].add();
    Recipe::writerecipes();
    Recipe::readrecipes();
    
    // Contrib::writecontribs();
    // Contrib::readcontribs();
    show_config();

    for (_, rec) in rdict {
        rec.display();
    }
    for (_, ctb) in cdict {
        ctb.display();
    }

    show_config();
    
    // RecipeConfig::write();
    
}
