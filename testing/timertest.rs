#![allow(non_snake_case)]

use std::time::Duration;
use tokio::time::{interval,Interval};

#[tokio::main]
async fn main()
{
    println!("allo");

    deployWatchTimer().await;
}

async fn deployWatchTimer()
{
    let mut timer:Interval=interval(Duration::from_secs(1));

    loop
    {
        timer.tick().await;
        println!("ticking");
    }
}