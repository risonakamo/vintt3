// functions for reading vintt config from file

#![allow(non_snake_case)]

use std::fs::File;
use std::io::BufReader;
use std::result::Result;

use crate::VinttConfig::VinttConfig;
use crate::types::errors::SerdeFileError;

/// read vintt config from yaml file
fn getVinttConfig(target:&str)->Result<VinttConfig,SerdeFileError>
{
    let file:File=File::open(target)?;
    let reader:BufReader<File>=BufReader::new(file);
    let config:VinttConfig=serde_yaml::from_reader(reader)?;
    return Ok(config);
}