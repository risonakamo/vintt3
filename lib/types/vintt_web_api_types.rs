use serde::{Deserialize,Serialize};
use std::collections::HashMap;

/// request to set category
#[derive(Deserialize,Debug)]
pub struct SetCategoryReq
{
    category:String
}

/// current watch information
#[derive(Serialize)]
pub struct CurrentWatch
{
    name:String,

    currentTime:u64,
    currentCategory:String,

    // all categories of the current watch and their times
    categories:HashMap<String,u64>
}