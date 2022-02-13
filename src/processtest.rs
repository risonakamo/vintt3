#![allow(non_snake_case)]
#![allow(unused_variables)]

use sysinfo::{System,SystemExt,ProcessExt};
use tokio::time::{interval,Interval};
use std::time::Duration;
use std::collections::HashSet;

#[tokio::main]
async fn main()
{
    println!("waiting for process...");

    let foundProcess:String=waitForAProcess(vec![
        "notepad.exe",
        "wt.exe"
    ]).await;

    println!("found {}",foundProcess);
}

/// wait for a process in the given list to exist. CONSUMES the input vector
async fn waitForAProcess(targets:Vec<&str>)->String
{
    let mut system:System=System::new_all();
    let mut timer:Interval=interval(Duration::from_secs(3));

    let targetSet:HashSet<&str>=HashSet::from_iter(targets);

    loop
    {
        timer.tick().await;
        system.refresh_processes();

        let foundProcess:String=someProcessExists(&targetSet,&system);

        if foundProcess.len() > 0
        {
            return foundProcess;
        }
    }
}

/// using a sysinfo system, see if a process in the process targets exists. if not, returns empty string
fn someProcessExists(processTargets:&HashSet<&str>,system:&System)->String
{
    for (i,x) in system.processes()
    {
        if processTargets.contains(x.name())
        {
            return x.name().to_string();
        }
    }

    return "".to_string();
}