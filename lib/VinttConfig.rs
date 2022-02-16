#![allow(non_snake_case)]

use std::collections::{HashMap,HashSet};
use serde::{Deserialize};

/// full vintt config yml file
#[derive(Deserialize,Debug)]
pub struct VinttConfig
{
    // key is process name
    pub track_items:HashMap<String,VinttItem>
}

#[derive(Deserialize,Debug,Clone,Default)]
pub struct VinttItem
{
    pub display_name:String,
    pub categories:Vec<String>
}

impl VinttConfig
{
    /// get all track processes as Set
    pub fn getProcessNames(&self)->HashSet<String>
    {
        return HashSet::from_iter(self.track_items.iter().map(
            |(i,_):(&String,&VinttItem)|->String {
                return i.clone();
            }
        ));
    }
}