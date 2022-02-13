use std::fs::File;
use std::io;
use std::io::BufReader;
use std::error;
use std::result;

use vintt3::types::vintt_config_types::VinttConfig;

fn main()->result::Result<(),Box<dyn error::Error>>
{
    let file:File=File::open("config/vintt_config.yml")?;
    let reader=BufReader::new(file);
    let config:VinttConfig=serde_yaml::from_reader(reader)?;

    println!("{:?}",config);

    return Ok(());
}