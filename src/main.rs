extern crate chrono;
mod mortgage;
mod api;
use api::api::{
    payment_schedule, index, overpayment
};
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    println!("Server started...");
    HttpServer::new(|| {
        App::new()
            .service(payment_schedule)
            .service(overpayment)
            .route("/", web::get().to(index))
    })
    .bind(("192.168.200.92", 8080))?
    .run()
    .await
}
