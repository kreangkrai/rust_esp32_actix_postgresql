use tokio;
use crate::databases::db::DB;
use tokio_postgres::{NoTls};
use crate::models::{Data};
use chrono::{DateTime,Utc};

pub async fn gets(n :String)->Result<Vec<Data>,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut datas :Vec<Data> = vec![];
    let mut id:i32;
    let mut device:String;
    let mut value:f32;
    let mut date:DateTime<Utc>;
    let rows = client.query("SELECT * FROM log_datas WHERE date >= NOW() - ($1 ||' minutes')::INTERVAL order by date desc;", &[&n]).await?;
    for row in rows{
        id = row.get(0);
        device = row.get(1);
        value = row.get(2);
        date = row.get(3);
                        
        datas.push(Data{id:id,device:device,value:value,date:date.to_string()});
    } 
    Ok(datas)      
}