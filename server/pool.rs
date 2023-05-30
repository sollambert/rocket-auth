use tokio_postgres::{NoTls, Error, Client, Row, types::ToSql};
use std::env;

fn get_connection_string() -> String {
    let (_, connection_string) = env::vars().into_iter().find(
        |(key, _)| key == "DB_CONNECTION_STRING").ok_or(()).unwrap();
    connection_string
}

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
    tokio_postgres::connect(&get_connection_string(), NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}