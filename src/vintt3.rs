#![allow(non_snake_case)]

use std::env::current_exe;
use std::path::PathBuf;
use warp::Filter;

use vintt3::VinttWatcher::VinttWatcher;
use vintt3::apis::vintt_config_api::getVinttConfig;
use vintt3::types::vintt_web_api_types::SetCategoryReq;

#[tokio::main]
async fn main()
{
    let currentDir:PathBuf=current_exe().unwrap().parent().unwrap().to_path_buf();

    let mut watcher:VinttWatcher=VinttWatcher::new(currentDir.join("time.yml").to_str().unwrap());

    watcher.watch(getVinttConfig(
        currentDir.join("vintt_config.yml").to_str().unwrap()
    ).unwrap());

    runWarp(&mut watcher).await;
}

/// run warp apis
async fn runWarp(watcher:&mut VinttWatcher)
{
    let root=warp::path::end().map(|| {
        return "hey";
    });

    // /get-watch
    // get current watch information
    let getWatch=warp::path!("get-watch")
        .and(warp::get())
        .map(|| {
            return "get-watch";
        });

    // /set-category (body)
    // set category of current watch program
    let setCategory=warp::path!("set-category")
        .and(warp::post())
        .and(warp::body::json())
        .map(|setCategoryReq:SetCategoryReq| {
            println!("set category {:?}",setCategoryReq);
            return "hello";
        });

    let routes=root
        .or(getWatch)
        .or(setCategory);

    warp::serve(routes).run((
        [0,0,0,0],
        4200
    )).await;
}