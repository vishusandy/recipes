
use {CFG, RECIPELIST, RECIPEDICT, CONTRIBLIST, CONTRIBDICT, ALLTAGS};
use std::mem;
use recipe_structs::*;
use entries::*;
use helpers::*;
use chrono::NaiveDate;


impl RecipeConfig {
    
    pub fn nexttid() -> u16 {
        let mut cfg: &mut RecipeConfig;
        let mut t: u16 = 0;
        unsafe {
            cfg = mem::transmute(CFG);
        }
        t = cfg.ai_tid;
        cfg.ai_tid += 1;
        t
    }
    
    pub fn previewtid() -> u16 {
        unsafe {
            (*CFG).ai_tid
        }
    }
    
    pub fn inctid() -> u16 {
        let mut cfg: &mut RecipeConfig;
        unsafe {
            cfg = mem::transmute(CFG);
        }
        cfg.ai_tid += 1;
        cfg.ai_tid
    }
    
    
    pub fn nextrid() -> u32 {
        let mut cfg: &mut RecipeConfig; 
        let mut t: u32 = 0;
        unsafe {
            cfg = mem::transmute(CFG);
        }
        t = cfg.ai_rid;
        cfg.ai_rid += 1;
        t
    }
    
    pub fn nextcid() -> u32 {
        let mut cfg: &mut RecipeConfig;
        let mut t: u32 = 0;
        unsafe {
            cfg = mem::transmute(CFG);
            t = (*CFG).ai_cid;
        }
        cfg.ai_cid += 1;
        t
    }
    
    pub fn previewrid() -> u32 {
        unsafe {
            (*CFG).ai_rid
        }
    }
    
    pub fn previewcid() -> u32 {
        unsafe {
            (*CFG).ai_cid
        }
    }
    
    pub fn incrid() -> u32 {
        let mut cfg: &mut RecipeConfig; 
        unsafe {
            cfg = mem::transmute(CFG);
        }
        cfg.ai_rid += 1;
        cfg.ai_rid
    }
    
    pub fn inccid() -> u32 {
        let mut cfg: &mut RecipeConfig;
        unsafe {
            cfg = mem::transmute(CFG);
        }
        cfg.ai_cid += 1;
        cfg.ai_cid
    }

}

