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
mod recipe_structs;
mod autoinc;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::mem;

use helpers::*;
use recipe_structs::*;
use autoinc::*;
use entries::*;


fn main() {
    
    let mut rcfg: RecipeConfig = RecipeConfig::config();
    // static mut CFG: *const u32 = &mut rcfg as *const u32;
    // static mut CFG: *mut u32 = &mut rcfg;
    static mut CFG: *const RecipeConfig = 0 as *const RecipeConfig;
    unsafe {
        CFG = mem::transmute(&rcfg);
    }
    
    
    rcfg.show_config();
    
    
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
    
    
    let mut rlist: Vec<Recipe> = Vec::new();
    
    recipelist[0].add(&mut rcfg, &mut rlist);
    recipelist[1].add(&mut rcfg, &mut rlist);
    
    // rlist.push(Recipe::add(&rlist, recipelist[0]));
    // rlist.push(Recipe::add(&rlist, recipelist[1]));
    
    // let mut recipelist2: Vec<Recipe> = Vec::new();
    
    // Recipe::writerecipes(&mut recipelist);
    // Recipe::readrecipes(&mut recipelist);
    
    Recipe::writerecipes(&mut rcfg, &mut rlist);
    Recipe::readrecipes(&mut rcfg, &mut rlist);
    
    for item in rlist {
        item.display();
    }
    
    
    rcfg.show_config();
    unsafe {
        println!("\n\nConfig[ai_rid] = {}\n", (*CFG).ai_rid);
    }
    /*
    let mut buf = Vec::new();
    recipelist.serialize(&mut Serializer::new(&mut buf)).unwrap();
    println!("Serialize Item buffer: \n{:?}\n", buf);
    */
    /*
    let mut ds = Deserializer::new(&buf[..]);
    let mut uns: Vec<Recipe> = Deserialize::deserialize(&mut ds).unwrap();
    for item in uns {
        item.display();
    }
    */
    
    
    
    
    
    
    
}
