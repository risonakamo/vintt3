use warp::Filter;

use vintt3::VinttWatcher::VinttWatcher;
use vintt3::apis::vintt_config_api::getVinttConfig;

#[tokio::main]
async fn main()
{
    tokio::spawn(async {
        let mut watcher:VinttWatcher=VinttWatcher::new("time.yml");
        watcher.watch(getVinttConfig("./config/vintt_config.yml").unwrap()).await;
    });

    let test=warp::path("test").map(||->String {
        return "hello".to_string();
    });

    let routes=warp::get().and(test);

    warp::serve(routes).run((
        [0,0,0,0],
        4200
    )).await;
}