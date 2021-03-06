#![allow(non_snake_case)]

use std::env::current_exe;
use std::path::PathBuf;
use warp::Filter;
use std::sync::{Arc,Mutex};
use log::info;

use vintt3::VinttWatcher::{VinttWatcher,CurrentWatch};
use vintt3::apis::vintt_config_api::getVinttConfig;
use vintt3::types::vintt_web_api_types::SetCategoryReq;

// debug paths
const VINTT_CONFIG_PATH:&str="../../testconfig/vintt_config.yml";
const TIMEFILE_PATH:&str="../../testconfig/time.yml";

// release paths
// const VINTT_CONFIG_PATH:&str="vintt_config.yml";
// const TIMEFILE_PATH:&str="time.yml";

#[tokio::main]
async fn main()
{
    pretty_env_logger::init();

    let currentDir:PathBuf=current_exe().unwrap().parent().unwrap().to_path_buf();

    let watcher:VinttWatcher=VinttWatcher::new(currentDir.join(TIMEFILE_PATH).to_str().unwrap());

    watcher.watch(getVinttConfig(
        currentDir.join(VINTT_CONFIG_PATH).to_str().unwrap()
    ).unwrap());

    runWarp(watcher).await;
}

/// run warp apis
async fn runWarp(watcher:VinttWatcher)
{
    let watcherArc=Arc::new(Mutex::new(watcher));

    let staticWebRoot=warp::get().and(warp::fs::dir("vintt3-web/build"));

    // /get-watch
    // get current watch information
    let getWatch=warp::path!("get-watch")
        .and(warp::get())
        .map(move || {
            info!("route: get-watch");
            let curWatch:CurrentWatch=watcherArc.lock().unwrap().getCurrentWatch();
            return warp::reply::json(&curWatch);
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

    let routes=staticWebRoot
        .or(getWatch)
        .or(setCategory)
        .with(warp::log("warp"));

    warp::serve(routes).run((
        [0,0,0,0],
        80
    )).await;
}