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
    /* unsafe {
        println!("Config:\nnumcontribs = {}\nnumrecipes = {}\nnextrid = {}\nnextcid = {}", rcfg.num_contribs, rcfg.num_recipes, rcfg.ai_rid, rcfg.ai_cid);
        println!("Config:\nnumcontribs = {}\nnumrecipes = {}\nnextrid = {}\nnextcid = {}", num_contribs, num_recipes, ai_rid, ai_cid);
         static mut rcfg: RecipeConfig = helpers::config();
        println!("Config:\nnumcontribs = {}\nnumrecipes = {}\nnextrid = {}\nnextcid = {}", rcfg.num_contribs, rcfg.num_recipes, rcfg.ai_rid, rcfg.ai_cid);
    } */
    
    
    let mut rcfg: RecipeConfig;
    rcfg = cfg();
    show_config(&rcfg);
}
