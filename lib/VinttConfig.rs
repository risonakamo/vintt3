use std::collections::{HashMap,HashSet};
use serde::{Deserialize};

#[derive(Deserialize,Debug)]
pub struct VinttConfig
{
    track_items:HashMap<String,VinttItem>
}

#[derive(Deserialize,Debug)]
struct VinttItem
{
    display_name:String,
    categories:Vec<String>
}

impl VinttConfig
{
    /// get all track processes as Set
    fn getProcessNames(&self)->HashSet<String>
    {
        return HashSet::from_iter(self.track_items.iter().map(
            |(i,_):(&String,&VinttItem)|->String {
                return i.clone();
            }
        ));
    }
}