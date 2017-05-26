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
mod recipe_impl;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

use helpers::*;
use recipe_structs::*;
use recipe_impl::*;
use entries::*;


fn main() {
    
    let mut rcfg: RecipeConfig = RecipeConfig::config();
    rcfg.show_config();
    
    let mut recipelist: Vec<Recipe> = vec![
        Recipe {
            rid: 1u32,
            title: String::from("Lasagna"),
            date: String::from("2017-05-26"),
            contributor: 30u32,
            ingredients: String::from("Lasagna Noodles\nTomatoes\nCheeses"),
            directions: String::from("1. Cook noodles\n2. Add ingredients\n3. Cook other stuff\n4. More and really long and complicated stuff that requires lots of cooking and patience and requires a cooking level of 99\n5. This is the final step."),
        },
        Recipe {
            rid: 2u32,
            title: String::from("Pizza"),
            date: String::from("2017-05-25"),
            contributor: 22u32,
            ingredients: String::from("Pizza Dough\n\tFlour\n\tSemolina Flour\n\tEggs\n\tWater\n\tOil\nTomatoes\nCheese"),
            directions: String::from("1. Add semolina flour and flour and eggs and oil\n2. Make mixture into dough\n3. Make tomatoes into sauce\n4. Add sauce and cheese and any toppings to the pizza\n5. Bake in oven at ow my hand tempatures\n6. Die from happiness"),
        },
    ];
    Recipe::writerecipes(&mut recipelist);
    let mut recipelist2: Vec<Recipe> = Vec::new();
    Recipe::readrecipes(&mut recipelist2);
    
    for item in recipelist2 {
        item.display();
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
