use std::time::Duration;

#[tokio::main]
async fn main() {
    console_subscriber::init();

    let mut task_id = 0_u64;

    loop {
        task_id += 1;

        tokio::spawn(async move {
            println!("Task {task_id} começou");

            tokio::time::sleep(Duration::from_secs(1)).await;

            println!("Task {task_id} terminou");
        });

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}