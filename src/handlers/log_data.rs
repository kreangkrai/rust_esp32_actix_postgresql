use actix_web::{http,web,HttpResponse,HttpRequest};
use crate::repository::{log_data};
use crate::errors::MyError;
pub async fn get_datas(req: HttpRequest,n :web::Path<String>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = log_data::gets(n.into_inner()).await?;   
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}