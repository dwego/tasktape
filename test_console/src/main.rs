use std::time::Duration;

#[tokio::main]
async fn main() {
    console_subscriber::init();

    loop {
        tokio::spawn(async {
            tokio::time::sleep(Duration::from_secs(1)).await;
        });

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}