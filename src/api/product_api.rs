use actix_web::HttpResponse;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, Result};
use tokio;
use std::thread;
use std::time::{Duration, Instant};
use diesel::prelude::*;
use crate::models::productmodel::*;
use diesel::{ Connection, prelude::* };
use diesel::pg::PgConnection;
use dotenv::dotenv;

fn db_connect() -> PgConnection {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("Database Must Be Set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", &db_url))
}


#[post("/product/create")]
pub async fn save_product(product: web::Json<CreateProduct>) -> Result<HttpResponse> {

    use crate::schema::products::dsl::*;
    let mut connection = db_connect();

    diesel::insert_into(products)
        .values(&product.into_inner())
        .execute(& mut connection)
        .expect("Error inserting new product");
    println!("ok");

    Ok(HttpResponse::Ok().json("Product Create Success."))

}

#[get("/products")]
pub async fn get_product() -> Result<HttpResponse> {
    let tid_main = thread::current().id();
    println!("Main thread id = {:?}", tid_main);

    let start = Instant::now();

    for i in 0..10 {
        println!("Iteration {} of {:?} is starting to sleep", i, tid_main);
        thread::sleep(Duration::from_secs(1));
        println!("Iteration {} of {:?} has finished sleeping", i, tid_main);
    }

    let duration = start.elapsed();
    println!("All iterations have finished in {:.3} seconds.", duration.as_secs_f64());

    Ok(HttpResponse::Ok().json("get"))
}

#[get("/multi")]
pub async fn get_multi() -> Result<HttpResponse> {
    let mut handles = vec![];

    let tid_main = thread::current().id();
    println!("Main thread id = {:?}", tid_main);

    let start = Instant::now();

    for i in 0..10 {
        let handle = thread::spawn(move || {
            let tid = thread::current().id();
            println!("Thread {} (ID: {:?}) is sleeping", i, tid);
            thread::sleep(Duration::from_secs(1));
            println!("Thread {} (ID: {:?}) has woken up", i, tid);
        });
        handles.push(handle);
    }
    for handle in handles {
        // 各スレッドが終わるまで待機する
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("All threads have finished in {:.3} seconds.", duration.as_secs_f64());
    Ok(HttpResponse::Ok().json("get"))
}
