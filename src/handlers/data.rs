use actix_web::{http,web,HttpResponse,HttpRequest};
use crate::models::{Data};
use crate::repository::{data};
use crate::errors::MyError;
pub async fn get_datas(req: HttpRequest) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = data::gets().await?;   
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn get_data(req: HttpRequest,device :web::Path<String>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let data = data::getbydevice(device.into_inner()).await?;   
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
pub async fn add_data(req: HttpRequest,_data:web::Json<Data>) -> Result<HttpResponse,MyError>{
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            let p:Data = Data{id:0,device:_data.device.to_string(),value:_data.value,date:String::from("")};
            let data = data::insert(p).await?;
            return Ok(HttpResponse::Ok().json(data));
        }
    }
    Ok(HttpResponse::BadRequest().into())
}
