use actix_web::web;
use crate::handlers::{data,log_data};

pub fn config_app(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/datas")
        .service(
            web::resource("")
            .route(web::get().to(data::get_datas))
            .route(web::post().to(data::add_data)),    
        )
        .service(
            web::scope("/{device}")
            .service(
                web::resource("")
                .route(web::get().to(data::get_data)),
            )
        ),
    )
    .service(
        web::scope("/log_datas")
        .service(
            web::scope("/{minutes}")
            .service(
                web::resource("")
                .route(web::get().to(log_data::get_datas)),
            )           
        )
    );  
}