use tasktape::connection::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut _connection =
        Connection::connect("http://127.0.0.1:6669").await?;

    println!("Conectado!");

    let mut updates = _connection.updates().await?;

    while let Some(update) = updates.message().await? {
        println!("Update recebido: {update:#?}");
    }

    Ok(())
}