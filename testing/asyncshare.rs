#![allow(non_snake_case)]

use tokio::time::{interval,Interval,sleep};
use std::time::Duration;
use std::sync::{Mutex,Arc};
use tokio::task::JoinHandle;

struct TestClass
{
    text:Arc<Mutex<String>>
}

impl TestClass
{
    fn new()->Self
    {
        return Self {
            text:Arc::new(Mutex::new("hello".to_string()))
        };
    }

    fn watch(&self)->JoinHandle<()>
    {
        let textRef:Arc<Mutex<String>>=self.text.clone();

        return tokio::spawn(async move {
            let mut timer:Interval=interval(Duration::from_secs(3));

            loop
            {
                timer.tick().await;
                println!("tick: {}",textRef.lock().unwrap());
            }
        });
    }

    fn change(&mut self,newtext:&str)
    {
        let testArc:Arc<Mutex<String>>=self.text.clone();
        *(testArc.lock().unwrap())=newtext.to_string();
    }
}

#[tokio::main]
async fn main()
{
    let mut test:TestClass=TestClass::new();

    let watchThread:JoinHandle<()>=test.watch();

    sleep(Duration::from_secs(3)).await;
    println!("changing");
    test.change("hello2");

    tokio::join!(watchThread);
}