use tokio::time::{interval,Interval};
use std::time::Duration;
use std::collections::HashSet;

use crate::VinttConfig::{VinttConfig,VinttItem};
use crate::process_watch::waitForAProcess;
use crate::apis::vintt_time_api::incrementTime;

struct VinttWatcher
{
    timefile:String,

    trackItem:VinttItem,
    pub categories:HashSet<String>,

    currentCategory:String,
    elapsedTime:u64,
    categoryTime:u64
}

impl VinttWatcher
{
    pub fn new(timefile:&str)->Self
    {
        return Self {
            timefile:timefile.to_string(),

            trackItem:VinttItem::default(),
            categories:HashSet::default(),

            currentCategory:"".to_string(),
            elapsedTime:0,
            categoryTime:0
        };
    }

    /// begin main watch loop. when find a program from the vintt config, begins writing to
    /// time file.
    pub async fn watch(&mut self,config:&VinttConfig)
    {
        println!("watching...");

        // get all processes to watch for
        let configProcesses:Vec<&String>=config.track_items.keys().collect();

        // wait until found a process
        let foundProcess:String=waitForAProcess(configProcesses).await;

        // set the track item to be that item
        self.trackItem=config.track_items.get(&foundProcess).unwrap().clone();
        self.categories=HashSet::from_iter(self.trackItem.categories.clone().into_iter());

        let mut timer:Interval=interval(Duration::from_secs(60));

        loop
        {
            // every 1 min
            timer.tick().await;

            incrementTime(
                &foundProcess,
                &self.currentCategory,
                1,
                &self.timefile
            ).unwrap();
            self.elapsedTime+=1;
            self.categoryTime+=1;
        }
    }

    /// attempt to change current category. only allowed to change category to something that is
    /// valid for the current item
    pub fn changeCategory(&mut self,newCategory:&str)->Result<(),String>
    {
        if !self.categories.contains(newCategory)
        {
            return Err("INVALID_CATEGORY".to_string());
        }

        self.currentCategory=newCategory.to_string();
        self.categoryTime=0;
        return Ok(());
    }
}