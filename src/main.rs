/*
recipes
    title date contributor ingredients directions
contributors
    name city state comments
*/

extern crate regex;
extern crate chrono;

mod recipe_structs;
mod helpers;


use recipe_structs::*;


fn main() {
    let rcfg = helpers::config();
    println!("Config:\nnumcontribs = {}\nnumrecipes = {}\nnextrid = {}\nnextcid = {}", rcfg.num_contribs, rcfg.num_recipes, rcfg.ai_rid, rcfg.ai_cid);
}
