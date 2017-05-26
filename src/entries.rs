
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::mem;
use chrono::NaiveDate;
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
        let date: NaiveDate = NaiveDate::from_ymd(2017, 5, 24);
        /*let contributor: Contrib = Contrib {
            cid: 0,
            added: NaiveDate::from_ymd(2017, 5, 24),
            name: String::new(),
            city: String::new(),
            state: String::new(),
            comments: String::new(),
        }; */
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

    
    //write all recipes in a collection to file
    pub fn writerecipes(list: &mut Vec<Recipe>) -> bool {

        false
    }

    pub fn allrecipes(v: &mut Vec<Recipe>) -> bool {
        //todo add check if not exist then create file
        let mut f = BufReader::new(File::open("recipes.db").unwrap());
        let mut b = String::new();
        let mut success = false;
        f.read_to_string(&mut b).expect("Could not open recipe data.");
        v.clear();
        for record in f.split(30u8) {
            let mut r: Recipe = Recipe {
                                    rid: 0u32,
                                    title: String::new(),
                                    date: NaiveDate::from_ymd(1, 1, 1),
                                    contributor: 0u32,
                                    ingredients: String::new(),
                                    directions: String::new(),
            };
            // let mut cur = record.unwrap();
            // while let unit = cur.partition(|&n| n == 31u8) {}
            {
                let mut valu = record.unwrap();
                let mut cur = valu.split(|&c| c == 31u8);
                for unit in cur {
                    match unit {
                        x if x.starts_with(&[b'r', b'i', b'd', GSP]) => {
                            //0 1 2 3 [4 5 6 7] 8
                            // let rawid = &unit[4 .. 8];
                            // let rid = mem::transmute::<[u8; 4], u32>(*rawid);
                        },
                        x if x.starts_with(&[b't', b't', b'l', GSP]) => {},
                        x if x.starts_with(&[b'd', b'a', b't', GSP]) => {},
                        x if x.starts_with(&[b'c', b't', b'b', GSP]) => {},
                        x if x.starts_with(&[b'i', b'g', b'd', GSP]) => {},
                        x if x.starts_with(&[b'd', b'i', b'r', GSP]) => {},
                        _ => {},
                    }
                    let prefix = 0;
                }
            }
            //let units = record.split(|c| c == 31u8);
            
            //let rec: Vec<_> = record.split().collect();
            // for unit in record.unwrap().split(31u8) {
                // match &unit[0 .. 4] {
             /*match units {/*
                    x if x == "rid" => {
                        r.rid = 0
                    },*//*
                    "ttl" => {},
                    "dat" => {},
                    "ctb" => {},
                    "igd" => {},
                    "dir" => {},
 */                  None  => {},
                     _     => {},
                
            }*/
            if r.rid != 0 && r.title != "" && r.date != NaiveDate::from_ymd(1, 1, 1) && r.contributor != 0 && r.ingredients != "" && r.directions != "" {
                v.push(r);
                if !success {
                    success = true;
                }
            }
        }
        success
    }
}




