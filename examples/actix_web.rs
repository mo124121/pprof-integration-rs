use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    let mut data: Vec<u64> = (0..10000).collect();

    for _ in 0..1000 {
        data.sort();
        data.reverse();
    }

    HttpResponse::Ok().body("Sorting completed")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(pprof_integration::frameworks::actix_web::configure)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
