#![allow(non_snake_case)]

use std::env::current_exe;
use std::path::PathBuf;
use warp::Filter;

use vintt3::VinttWatcher::VinttWatcher;
use vintt3::apis::vintt_config_api::getVinttConfig;

#[tokio::main]
async fn main()
{
    let currentDir:PathBuf=current_exe().unwrap().parent().unwrap().to_path_buf();

    let mut watcher:VinttWatcher=VinttWatcher::new(currentDir.join("time.yml").to_str().unwrap());
    let watcherTask=watcher.watch(getVinttConfig(
        currentDir.join("vintt_config.yml").to_str().unwrap()
    ).unwrap());

    let test=warp::path("test").map(||->String {
        return "hello".to_string();
    });

    let routes=warp::get().and(test);

    warp::serve(routes).run((
        [0,0,0,0],
        4200
    )).await;
}