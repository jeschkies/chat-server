use serde::{Deserialize, Serialize};

use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Message {
    author: String,
    content: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // GET /messages
    let messages = warp::get()
        .and(warp::path("hi")
        .map(|| {
            let message = Message {
                author: "Karsten".to_owned(),
                content: "Hello World!".to_owned(),
            };
            warp::reply::json(&vec![message])
        }));

    // POST /message
    let new_message = warp::post()
        .and(warp::path("message"))
        .and(warp::body::json())
        .map(|mut message: Message| {
            warp::reply::with_status(warp::reply(), warp::http::StatusCode::CREATED)
        });

    let routes = messages.or(new_message);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
