/*
recipes
    title date contributor ingredients directions
contributors
    name city state comments
*/
extern crate regex;
use regex::{Regex, RegexSet};

struct Date {
    year(u16),
    month(u8),
    day(u8)
}
impl Date {
    fn string(&self) -> String { 
        format!("{:04}-{:02}-{02}", self.year, self.month, self.day)
    }
    fn is_date(datestr: String) -> bool {
        let ymd = Regex::new("(?P<year>[0-9]{2}(?:[0-9]{2})?)[\./-](?P<month>[0-1]?[0-9])[\./-](?P<day>[0-3]?[0-9])").unwrap();
        let mdy = Regex::new("(?P<month>[0-1]?[0-9])[\./-](?P<day>[0-3]?[0-9])[\./-](?P<year>[0-9]{2}(?:[0-9]{2})?)").unwrap();
        match datestr {
            ymd.is_match(s) => true,
            mdy.is_match(s) => true,
            _ => false
        }
    }
    fn date(datestr: String) -> Option<Date> {
        /* let regset = RegexSet::new(&[
            r"^(?P<year>[0-9]{2}(?:[0-9]{2})?)[\./-](?P<month>[0-9]{1,2})[\./-](?P<day>[0-9]{1,2})$",           
            r"^(?P<year>[0-9]{2}(?:[0-9]{2})?)[\./-](?P<month>[0-1]?[0-9])[\./-](?P<day>[0-3]?[0-9])$",       //yy[yy]-[m]m-[d]d
            r"^(?P<month>[0-1]?[0-9])[\./-](?P<day>[0-3]?[0-9])[\./-](?P<year>[0-9]{2}(?:[0-9]{2})?)$",       //[m]m-[d]d-yy[yy]
            ]).unwrap();
        let matches: Vec<_> = set.matches(datestr).into_iter().collect();
        for (i, v) in matches.enumerate()
        */ 
        
        
    }
    // fn eq(&self, r: &self) -> bool {}
    // fn gt(&self, r: &self) -> bool {}
    // fn lt(&self, r: &self) -> bool {}
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}



struct Recipe {
    Rid(u32),
    Title(String),
    Date(),
    Contributor(Contrib),
    Ingredients(String),
    Directions(String),
}

struct Contrib {
    Cid(u32),
    Added(Date),
    Name(String),
    City(String),
    State(String),
    Comments(String),
    
}



fn main() {
    println!("Hello, world!");
}
