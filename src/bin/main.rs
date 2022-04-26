use actix_web::{middleware,App,HttpServer};
use rust_esp32_actix_postgresql::app_config::config_app;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(||{
        App::new()
        .configure(config_app)
        .wrap(middleware::Logger::default())
    })
    .bind(("172.20.10.8",8082))?
    .run()
    .await
}