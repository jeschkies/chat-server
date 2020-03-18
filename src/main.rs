
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let db = models::empty_db();

    let api = filters::all(db);

    let routes = api;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod filters {
    use super::handlers;
    use super::models::Db;
    use warp::Filter;

    pub fn all(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        list_messages(db.clone()).or(new_message(db.clone()))
    }

    // GET /messages
    fn list_messages(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path("messages"))
            .and(with_db(db))
            .and_then(handlers::list_messages)
    }

    // POST /message
    fn new_message(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::post()
            .and(warp::path("message"))
            .and(warp::body::json())
            .and(with_db(db))
            .and_then(handlers::create_message)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}

mod handlers {
    use super::models::{Db, Message};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn create_message(message: Message, db: Db) -> Result<impl warp::Reply, Infallible> {
        let mut messages = db.lock().await;

        messages.push(message);
        Ok(StatusCode::CREATED)
    }

    pub async fn list_messages(db: Db) -> Result<impl warp::Reply, Infallible> {
        let messages = db.lock().await;

        Ok(warp::reply::json(&messages.clone()))
    }
}

mod models {
    use std::sync::Arc;
    use serde::{Deserialize, Serialize};
    use tokio::sync::Mutex;

    pub type Db = Arc<Mutex<Vec<Message>>>;

    #[derive(Deserialize, Serialize, Clone)]
    pub struct Message {
        author: String,
        content: String,
    }

    pub fn empty_db() -> Db {
        Arc::new(Mutex::new(Vec::new()))
    }
}
