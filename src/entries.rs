
use chrono::NaiveDate;
use recipe_structs::*;
use recipe_impl::*;
use helpers::*;

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
        let contributor: Contrib = Contrib {
            cid: 0,
            added: NaiveDate::from_ymd(2017, 5, 24),
            name: String::new(),
            city: String::new(),
            state: String::new(),
            comments: String::new(),
        };
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

    pub fn allrecipes(list: &mut Vec<Recipe>) -> bool {
        
        false
    }
}

