use crate::VinttConfig::{VinttConfig,VinttItem};
use crate::process_watch::waitForAProcess;

struct VinttWatcher
{
    trackItem:VinttItem
}

impl VinttWatcher
{
    /// creates a vintt watcher from vintt config
    async fn new(config:VinttConfig)->Self
    {
        let configProcesses:Vec<&String>=config.track_items.keys().collect();
        let foundProcess:String=waitForAProcess(configProcesses).await;

        return Self {
            trackItem:config.track_items.get(&foundProcess).unwrap().clone()
        };
    }
}