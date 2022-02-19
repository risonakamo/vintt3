#![allow(non_snake_case)]

use std::env::current_exe;
use std::path::PathBuf;

use vintt3::VinttWatcher::VinttWatcher;
use vintt3::apis::vintt_config_api::getVinttConfig;

#[tokio::main]
async fn main()
{
    let currentDir:PathBuf=current_exe().unwrap().parent().unwrap().to_path_buf();

    let mut watcher:VinttWatcher=VinttWatcher::new(currentDir.join("time.yml").to_str().unwrap());

    watcher.watch(getVinttConfig(
        currentDir.join("vintt_config.yml").to_str().unwrap()
    ).unwrap()).await;
}