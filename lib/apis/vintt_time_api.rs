// functions for interacting with vintt time file

#![allow(non_snake_case)]

use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::result::Result;
use std::collections::HashMap;

use crate::types::vintt_time::{VinttTimeFile,VinttTime};
use crate::types::errors::SerdeFileError;

pub fn incrementTime(
    process:&str,
    category:&str,
    time:u64,
    filepath:&str
)->Result<(),SerdeFileError>
{
    let mut timeFile:VinttTimeFile=getVinttTimeFile(filepath);

    if !timeFile.tracked_items.contains_key(process)
    {
        timeFile.tracked_items.insert(process.to_string(),VinttTime {
            total_time:0,
            categories:HashMap::new()
        });
    }

    let currentTrackItem:&mut VinttTime=timeFile.tracked_items.get_mut(process).unwrap();
    currentTrackItem.total_time+=time;

    if category.len() > 0
    {
        let categoryTime:u64=match currentTrackItem.categories.get(category) {
            Some(x) => x.clone(),
            None => 0
        };

        currentTrackItem.categories.insert(category.to_string(),categoryTime+time);
    }

    writeVinttTimeFile(
        filepath,
        &timeFile
    )?;

    return Ok(());
}

/// get vintt time file. returns default if failed to find file
fn getVinttTimeFile(path:&str)->VinttTimeFile
{
    let file:File=match File::open(path) {
        Err(_) => return VinttTimeFile::new(),
        Ok(x) => x
    };

    let reader:BufReader<File>=BufReader::new(file);

    let config:VinttTimeFile=match serde_yaml::from_reader(reader) {
        Err(_) => return VinttTimeFile::new(),
        Ok(x) => x
    };

    return config;
}

/// write vintt time to file. creates file if doesn't exist.
fn writeVinttTimeFile(path:&str,vinttTime:&VinttTimeFile)->Result<(),SerdeFileError>
{
    let file:File=OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

    serde_yaml::to_writer(file,vinttTime)?;

    return Ok(());
}

pub mod test
{
    use super::incrementTime;

    pub fn testTimeApi()
    {
        incrementTime(
            "notepad.exe",
            "something1",
            12,
            "out.yml"
        ).unwrap();
    }
}