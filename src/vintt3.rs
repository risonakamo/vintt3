#![allow(non_snake_case)]

use std::env::current_exe;
use std::path::PathBuf;
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::time::{sleep,Duration};

use vintt3::VinttWatcher::VinttWatcher;
use vintt3::apis::vintt_config_api::getVinttConfig;

#[tokio::main]
async fn main()
{
    let currentDir:PathBuf=current_exe().unwrap().parent().unwrap().to_path_buf();

    let watcher:VinttWatcher=VinttWatcher::new(currentDir.join("time.yml").to_str().unwrap());
    let watcherMutex=Arc::new(Mutex::new(watcher));
    let watcherMutex2=Arc::clone(&watcherMutex);

    let watcherTask=tokio::spawn(async move {
        watcherMutex.lock().await.watch(getVinttConfig(
            currentDir.join("vintt_config.yml").to_str().unwrap()
        ).unwrap());
    });

    sleep(Duration::from_secs(10)).await;
    println!("changing");
    watcherMutex2.lock().await.changeCategory("something").unwrap();

    println!("done changing");
    tokio::join!(watcherTask);
}