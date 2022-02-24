use std::sync::{Mutex,Arc};
use std::time::Duration;
use tokio::time::{interval,Interval};

struct TestClass
{
    text:String
}

impl TestClass
{
    fn new()->Self
    {
        return Self {
            text:"hello".to_string()
        };
    }

    fn updateText(&mut self,newtext:&str)
    {
        self.text=newtext.to_string();
    }
}

#[tokio::main]
async fn main()
{
    let testClass:TestClass=TestClass::new();
    let testClassRef=Mutex::new(Arc::new(testClass));

    let task=tokio::spawn(async move {
        let mut timer:Interval=interval(Duration::from_secs(3));
        loop
        {
            timer.tick().await;
            println!("value: {}",testClassRef.lock().unwrap().text);
        }
    });

    testClassRef.lock().unwrap().updateText("adasda");

    tokio::join!(task);
}