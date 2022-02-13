#![allow(non_snake_case)]
#![allow(unused_variables)]

use sysinfo::{System,SystemExt,ProcessExt,Process,Pid};
use std::collections::HashSet;

fn main()
{
    let mut system=System::new_all();
    system.refresh_processes();

    let targetExe:&str="notepad.exe";

    let processNames:HashSet<String>=HashSet::from_iter(
        system.processes().iter().map(|(i,x):(&Pid,&Process)|->String {
            return x.name().to_string();
        })
    );

    if processNames.contains(targetExe)
    {
        println!("found target");
    }
}