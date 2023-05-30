use tokio_postgres::{NoTls, Error, Client, Row, types::ToSql};

use crate::get_env;

const DB_CONN: &str = "DB_CONNECTION_STRING";

pub async fn query(query: &str, params: Vec<&(dyn ToSql + Sync)>) -> Result<Vec<Row>, Error> {
    let client = get_client().await;
    let statement = client.prepare(query).await?;
    let db_res = client.query(&statement, &params[..]).await?;
    Ok(db_res)
}

pub async fn execute(query: &str, params: Vec<&(dyn ToSql + Sync)>) -> Result<u64, Error> {
    let client = get_client().await;
    let statement = client.prepare(query).await?;
    let db_res = client.execute(&statement, &params[..]).await?;
    Ok(db_res)
}

async fn get_client() -> Client {
    let (client, connection) =
    tokio_postgres::connect(&get_env(DB_CONN), NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}