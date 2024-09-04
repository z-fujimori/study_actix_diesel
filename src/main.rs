mod api;
mod models;
mod schema;

use actix_web:: {
    HttpServer, App, web, HttpResponse, Responder
};

use api::product_api::*;

#[actix_web::main]
// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .app_data(tera)
            .service(save_product)
            .service(get_product)
            .service(get_multi)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
    
}