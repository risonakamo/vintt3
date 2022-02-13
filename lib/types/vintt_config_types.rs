use std::collections::HashMap;
use serde::{Deserialize};

#[derive(Deserialize,Debug)]
pub struct VinttConfig
{
    track_items:HashMap<String,VinttItem>
}

#[derive(Deserialize,Debug)]
pub struct VinttItem
{
    display_name:String,
    categories:Vec<String>
}