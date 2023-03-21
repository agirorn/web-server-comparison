#![allow(unused)] // silence unused warnings while exploring (to comment out)
use std::process;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
    extract::{Extension}
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
// use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use sqlx::PgPool;
use tower::ServiceBuilder;

// use axum::extract::Extension;

// #[derive(Debug, FromRow)]
#[derive(Debug)]
struct Counter {
	count: Option<i64>,
}

#[derive(Debug)]
struct Insert {
	time: String
}


#[derive(Clone)]
struct ApiContext {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();


	// let pool = PgPoolOptions::new()
	// 	.max_connections(5)
	// 	.connect("postgres://user:pass@localhost/rust")
	// 	.await;
    //
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let counts: Vec<Counter> = sqlx::query_as!(
       Counter,
       "select count(id) from events",
    )
       .fetch_all(&pool)
       .await.expect("Unable to query domains table");

    println!("{:?}", counts);

    sqlx::query!(
      "insert into events(time) values ($1);",
      "time"
    )
      .execute(&pool)
      .await.expect("Unable to insert a domain");

    // let rows = sqlx::query("SELECT * FROM events").fetch_all(&pool).await;
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/hello-world", get(hello_world))
        // `POST /users` goes to `create_user`
        .route("/save", post(save_event))
        .layer(
        ServiceBuilder::new()
            // The other reason for using a single object is because `AddExtensionLayer::new()` is
            // rather verbose compared to Actix-web's `Data::new()`.
            //
            // It seems very logically named, but that makes it a bit annoying to type over and over.
            .layer(Extension(ApiContext {
                pool,
            }))
            // // Enables logging. Use `RUST_LOG=tower_http=debug`
            // .layer(TraceLayer::new_for_http()),
    );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("App pid is {}", process::id());
    // tracing::debug!("listening on {}", addr);
    // println!("listening on {}", addr);
    println!("listening on 0.0.0.0:3000");
    // axum::Server::bind(&addr)
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// basic handler that responds with a static string
async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn save_event(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    ctx: Extension<ApiContext>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {

    // let counts: Vec<Counter> = sqlx::query_as!(
    //    Counter,
    //    "select count(id) from events",
    // )
    //    .fetch_all(&ctx.pool)
    //    .await.expect("Unable to query domains table");
    //
    // println!("{:?}", counts);

    sqlx::query!(
      "insert into events(time) values ($1);",
      "time"
    )
      .execute(&ctx.pool)
      .await.expect("Unable to insert a domain");

    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.key,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    key: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
