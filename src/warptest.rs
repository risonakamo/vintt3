use warp::Filter;

#[tokio::main]
async fn main()
{
    let test=warp::path("test").map(||->String {
        return "hello".to_string();
    });

    let routes=warp::get().and(test);

    warp::serve(routes).run((
        [0,0,0,0],
        4200
    )).await;
}