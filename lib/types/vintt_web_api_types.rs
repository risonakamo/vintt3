use serde::{Deserialize};

/// request to set category
#[derive(Deserialize,Debug)]
pub struct SetCategoryReq
{
    category:String
}