use vintt3::VinttWatcher::VinttWatcher;
use vintt3::apis::vintt_config_api::getVinttConfig;

#[tokio::main]
async fn main()
{
    let mut watcher:VinttWatcher=VinttWatcher::new("time.yml");

    watcher.watch(getVinttConfig("./config/vintt_config.yml").unwrap()).await;
}