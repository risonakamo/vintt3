use sysinfo::{System,SystemExt,ProcessExt};
use tokio::time::{interval,Interval};
use std::time::Duration;
use std::collections::HashSet;

/// wait for a process in the given list to exist. CONSUMES the input vector
pub async fn waitForAProcess(targets:Vec<&String>)->String
{
    let mut system:System=System::new_all();
    let mut timer:Interval=interval(Duration::from_secs(3));

    let targetSet:HashSet<&String>=HashSet::from_iter(targets);

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
fn someProcessExists(processTargets:&HashSet<&String>,system:&System)->String
{
    for (_,x) in system.processes()
    {
        if processTargets.contains(&x.name().to_string())
        {
            return x.name().to_string();
        }
    }

    return "".to_string();
}