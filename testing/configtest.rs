use std::fs::File;
use std::io::BufReader;
use std::result;
use serde_yaml;

use vintt3::VinttConfig::VinttConfig;
use vintt3::types::errors::SerdeFileError;

fn main()->result::Result<(),SerdeFileError>
{
    let file:File=File::open("config/vintt_config.yml")?;
    let reader=BufReader::new(file);
    let config:VinttConfig=serde_yaml::from_reader(reader)?;

    println!("{:?}",config);

    return Ok(());
}