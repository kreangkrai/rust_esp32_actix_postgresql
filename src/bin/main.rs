use actix_web::{middleware,App,HttpServer};
use actix_web_test::app_config::config_app;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(||{
        App::new()
        .configure(config_app)
        .wrap(middleware::Logger::default())
    })
    .bind(("172.20.10.4",8081))?
    .run()
    .await
}