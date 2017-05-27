    //serializer
    let mut buf = Vec::new();
    recipelist.serialize(&mut Serializer::new(&mut buf)).unwrap();
    println!("Serialize Item buffer: \n{:?}\n", buf);
    //deserializer
    let mut ds = Deserializer::new(&buf[..]);
    let mut uns: Vec<Recipe> = Deserialize::deserialize(&mut ds).unwrap();
    for item in uns {
        item.display();
    }
    
    

let mut rcfg: RecipeConfig = RecipeConfig::config();
// static mut CFG: *const u32 = &mut rcfg as *const u32;
// static mut CFG: *mut u32 = &mut rcfg;
// static mut GCFG: RecipeConfig = RecipeConfig::new();
unsafe {
    CFG = mem::transmute(&rcfg);
}

unsafe {
    println!("\n\nConfig[ai_rid] = {}\n", (*CFG).ai_rid);
}

pub fn setconfig(rcfg: &mut RecipeConfig) {
        unsafe {
            CFG = mem::transmute(&rcfg);
        }
} 

const GSP: u8 = 29;
const RSP: u8 = 30;
const USP: u8 = 31;
const SOT: u8 = 2;
const EOT: u8 = 3;

const IDX: char = '\u{0084}';
const SOS: char = '\u{0098}';
const EOS: char = '\u{009c}';
 
 
 
 
    pub fn readrecipe(id: u32) -> Recipe {
        let rid: u32 = 0;
        let title = String::new();
        let date = String::new(); // NaiveDate = NaiveDate::from_ymd(2017, 5, 24);
        let contributor: Contrib = Contrib {
            cid: 0,
            added: String::new(), // NaiveDate::from_ymd(2017, 5, 24),
            name: String::new(),
            city: String::new(),
            state: String::new(),
            comments: String::new(),
        }; 
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

    pub fn writerecipe(item: &Recipe) -> bool {
        let mut success = false;
        
        let mut buf = Vec::new();
        item.display();
        
        item.serialize(&mut Serializer::new(&mut buf)).unwrap();
        println!("Serialize Item buffer: \n{:?}\n", buf);
        
        success
    }
    
    //write all recipes in a collection to file
    pub fn writerecipes(list: &mut Vec<Recipe>) -> bool {
        let mut fs = File::create("recipes.db");
        let mut f: File;
        
        match fs {
            Err(e) => return false,
            Ok(r)  => { f = r; },
        }
        for item in list {
            
            
            // println!("Recipe {}\nTitle: {}\nDate: {}\nContributor: {}\nIngredients:\n{}\nDirections:\n{}\n", item.rid, item.title, item.date, item.contributor, item.ingredients, item.directions);
            // println!("Deserialize Item buffer:\n{:?}\n", uns);
            // f.write()
        }
        false
    }

    pub fn allrecipes(v: &mut Vec<Recipe>) -> bool {
        //todo add check if not exist then create file
        let mut f = BufReader::new(File::open("recipes.db").unwrap());
        let mut b = String::new();
        let mut success = false;
        f.read_to_string(&mut b).expect("Could not open recipe data.");
        v.clear();
        let mut buf = Vec::new();
        let mut ds = Deserializer::new(&buf[..]);
        let mut uns: Recipe = Deserialize::deserialize(&mut ds).unwrap();
        uns.display();
        
        success
    }
    
    
    
    




for record in f.split(30u8) {
            let mut r: Recipe = Recipe {
                                    rid: 0u32,
                                    title: String::new(),
                                    date: String::new(),
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
                        x if x.starts_with(&[b'r', b'i', b'd', GSP]) => {},
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
            if r.rid != 0 && r.title != "" && r.date != "" && r.contributor != 0 && r.ingredients != "" && r.directions != "" {
                v.push(r);
                if !success {
                    success = true;
                }
            }
        }