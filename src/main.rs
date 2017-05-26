/*
recipes
    title date contributor ingredients directions
contributors
    name city state comments
*/

extern crate regex;
extern crate chrono;

mod helpers;
mod entries;
mod recipe_structs;
mod recipe_impl;

use helpers::*;
use recipe_structs::*;
use recipe_impl::*;
use entries::*;


fn main() {
    
    let mut rcfg: RecipeConfig = RecipeConfig::config();
    rcfg.show_config();
    
    
}
