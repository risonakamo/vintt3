// functions for reading vintt config from file

#![allow(non_snake_case)]

use std::fs::File;
use std::io::BufReader;
use std::result::Result;
use std::process::exit;

use crate::VinttConfig::VinttConfig;
use crate::types::errors::SerdeFileError;

/// read vintt config from yaml file
pub fn getVinttConfig(target:&str)->Result<VinttConfig,SerdeFileError>
{
    let file:File=match File::open(target) {
        Ok(r) => r,
        Err(_) => {
            println!("vintt_config read error: could not open {}",target);
            exit(0);
        }
    };

    let reader:BufReader<File>=BufReader::new(file);

    let config:VinttConfig=match serde_yaml::from_reader(reader) {
        Ok(r) => r,
        Err(_) => {
            println!("vintt_config.yml yaml parse error");
            exit(0);
        }
    };

    return Ok(config);
}