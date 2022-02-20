use tokio::time::{interval,Interval};
use tokio::task::JoinHandle;
use std::time::Duration;
use std::collections::{HashSet,HashMap};
use std::sync::{Arc,Mutex};
use serde::Serialize;

use crate::VinttConfig::{VinttConfig,VinttItem};
use crate::process_watch::waitForAProcess;
use crate::apis::vintt_time_api::incrementTime;

const WRITE_INTERVAL:u64=5;

pub struct VinttWatcher
{
    timefile:Arc<Mutex<String>>,

    watchProgram:Arc<Mutex<String>>,
    pub categories:Arc<Mutex<HashSet<String>>>,

    currentCategory:Arc<Mutex<String>>,
    elapsedTime:Arc<Mutex<u64>>,
    categoryTime:Arc<Mutex<u64>>
}

/// current watch information
#[derive(Serialize,Default)]
pub struct CurrentWatch
{
    name:String,

    currentTime:u64,
    currentCategory:String,

    // all categories of the current watch and their times
    categories:HashMap<String,u64>
}

impl VinttWatcher
{
    /// new vintt watcher. give path to target output timefile
    pub fn new(timefile:&str)->Self
    {
        return Self {
            timefile:Arc::new(Mutex::new(timefile.to_string())),

            watchProgram:Arc::new(Mutex::new(String::default())),
            categories:Arc::new(Mutex::new(HashSet::default())),

            currentCategory:Arc::new(Mutex::new(String::default())),
            elapsedTime:Arc::new(Mutex::new(0)),
            categoryTime:Arc::new(Mutex::new(0))
        };
    }

    /// begin main watch loop. when find a program from the vintt config, begins writing to
    /// time file. CONSUMES config
    pub fn watch(&self,config:VinttConfig)->JoinHandle<()>
    {
        println!("watching...");

        let categoriesArc=self.categories.clone();
        let currentCategoryArc=self.currentCategory.clone();
        let elapsedTimeArc=self.elapsedTime.clone();
        let categoryTimeArc=self.categoryTime.clone();
        let timefileArc=self.timefile.clone();
        let watchProgramArc=self.watchProgram.clone();

        return tokio::spawn(async move {
            // get all processes to watch for
            let configProcesses:Vec<&String>=config.track_items.keys().collect();

            // wait until found a process
            let foundProcess:String=waitForAProcess(configProcesses).await;
            println!("tracking: {}",foundProcess);
            *(watchProgramArc.lock().unwrap())=foundProcess.clone();

            // set the track item to be that item
            let trackItem:VinttItem=config.track_items.get(&foundProcess).unwrap().clone();
            *(categoriesArc.lock().unwrap())=HashSet::from_iter(trackItem.categories.into_iter());

            let mut timer:Interval=interval(Duration::from_secs(WRITE_INTERVAL));

            loop
            {
                // every 1 min
                timer.tick().await;
                println!("writing");

                incrementTime(
                    &foundProcess,
                    &currentCategoryArc.lock().unwrap(),
                    1,
                    &timefileArc.lock().unwrap()
                ).unwrap();

                *(elapsedTimeArc.lock().unwrap())+=1;
                *(categoryTimeArc.lock().unwrap())+=1;
            }
        });
    }

    /// attempt to change current category. only allowed to change category to something that is
    /// valid for the current item
    pub fn changeCategory(&mut self,newCategory:&str)->Result<(),String>
    {
        if !self.categories.lock().unwrap().contains(newCategory)
        {
            return Err("INVALID_CATEGORY".to_string());
        }

        println!("changing category: {}",newCategory);
        *(self.currentCategory.lock().unwrap())=newCategory.to_string();
        *(self.categoryTime.lock().unwrap())=0;
        return Ok(());
    }

    /// get the current watch. name will be empty if there is no watch
    pub fn getCurrentWatch(&self)->CurrentWatch
    {
        if self.watchProgram.lock().unwrap().len() == 0
        {
            return CurrentWatch::default();
        }

        return CurrentWatch {
            name:self.watchProgram.lock().unwrap().clone(),

            currentTime:self.elapsedTime.lock().unwrap().clone(),
            currentCategory:self.currentCategory.lock().unwrap().clone(),

            categories:HashMap::default()
        };
    }
}