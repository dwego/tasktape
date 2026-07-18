use tasktape::connection::Connection;
use tasktape::viewer::Viewer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut _connection =
        Connection::connect("http://127.0.0.1:6669").await?;

    println!("Conectado!");

    let mut updates = _connection.updates().await?;

    let mut sequence = 0_u64;

    while let Some(update) = updates.message().await? {
        Viewer::print_update(sequence, &update);
        sequence += 1;
    }

    Ok(())
}