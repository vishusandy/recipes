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

    static mut CFG: *const RecipeConfig = 0 as *const RecipeConfig;

fn main() {
    
    let mut rcfg: RecipeConfig = RecipeConfig::config();
    
    unsafe {
        CFG = mem::transmute(&rcfg);
        // (*CFG).num_contribs = 100u32;
    }
    rcfg.num_contribs = 100u32;
    
    setconf(&rcfg);
    
    showconf();
    testcontrib(&mut rcfg);
    
    show_config();
    // rcfg.show_config();
    
    
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
    
    // for item in rlist {
        // item.display();
    // }
    
    
    show_config();
    showconf();
    
    // show_config();
    // rcfg.show_config();

    
    
    
    
    
    
    
}
