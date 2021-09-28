mod repository;
mod user;
mod health;
mod v1;
mod db;

use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

use actix_web::{App, HttpServer, web};
use repository::MemoryRepository;
use tokio_postgres::NoTls;
use crate::db::handlers::add_user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init env vars 
    dotenv::dotenv().ok();
    // building address
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    // let address = format!("127.0.0.1:{}", port);

    let thread_counter = Arc::new(AtomicU16::new(1));
    let repo = web::Data::new(MemoryRepository::default());

    let config = crate::db::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move || {
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);

        App::new()
            .data(thread_index)
            .data(pool.clone())
            .app_data(repo.clone())
            .configure(v1::service::<MemoryRepository>)
            .configure(health::service)
            .service(web::resource("/db").route(web::post().to(add_user)))
    })
    .bind(config.server_addr.clone())
    .unwrap_or_else(|err| panic!("🔥🔥🔥 No se pudo iniciar el servidor en el puerto {}: {:?}", port, err))
    .run()
    .await
}
