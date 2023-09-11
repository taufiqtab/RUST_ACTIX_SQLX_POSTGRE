use actix_web::{get, post, web, web::Data, App, HttpResponse, 
HttpServer, Responder};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres, FromRow};

use dotenv::dotenv;

use serde::{Deserialize, Serialize};

pub struct AppState {
    db : Pool<Postgres>
}

#[derive(Serialize, FromRow)]
struct Customer{
    customer_code : String,
    bp_group : String,
    customer_name : String
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/uci")]
async fn uci_call() -> impl Responder {
    HttpResponse::Ok().body("Aku padamu !")
}

#[get("/customers")]
async fn get_customers(state: Data<AppState>) -> impl Responder {
    
    match sqlx::query_as::<_, Customer>("SELECT customer_code, bp_group, customer_name from customer")
        .fetch_all(&state.db)
        .await{
            Ok(customer) =>     HttpResponse::Ok().json(customer),
            Err(_) => HttpResponse::NotFound().json("No Customer Found"),
        }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await
        .expect("Error Building a connection pool");

    HttpServer::new(move|| {
        App::new()
            .app_data(Data::new(AppState{db:pool.clone()}))
            .service(uci_call)
            .service(get_customers)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
