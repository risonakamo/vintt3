use std::collections::HashMap;
use serde::{Deserialize,Serialize};

/// full vintt time file
#[derive(Deserialize,Serialize)]
pub struct VinttTimeFile
{
    // key is process name
    pub tracked_items:HashMap<String,VinttTime>
}

/// time information for single process
#[derive(Deserialize,Serialize)]
pub struct VinttTime
{
    // time in minutes
    pub total_time:u64,

    // time per category (key) in minutes
    pub categories:HashMap<String,u64>
}

impl VinttTimeFile
{
    pub fn new()->Self
    {
        return Self {
            tracked_items:HashMap::new()
        };
    }
}