use console_api::instrument::Update;

pub struct Viewer;

impl Viewer {
    pub fn print_update(sequence: u64, update: &Update) {
        println!("\n========================");
        println!("UPDATE #{sequence}");
        println!("========================");

        match &update.now {
            Some(timestamp) => {
                println!(
                    "Timestamp: {}.{:09}",
                    timestamp.seconds,
                    timestamp.nanos
                );
            }
            None => {
                println!("Timestamp: indisposition");
            }
        }

        match &update.task_update {
            Some(tasks) => {
                println!("\nTasks:");
                println!("  new: {}", tasks.new_tasks.len());
                println!("  stats updated: {}", tasks.stats_update.len());
                println!("  dropped events: {}", tasks.dropped_events);
            }
            None => {
                println!("\nTasks: no update");
            }
        }

        match &update.resource_update {
            Some(resources) => {
                println!("\nResources:");
                println!("  new: {}", resources.new_resources.len());
                println!(
                    "  stats updated: {}",
                    resources.stats_update.len()
                );
                println!("  poll ops: {}", resources.new_poll_ops.len());
                println!("  dropped events: {}", resources.dropped_events);
            }
            None => {
                println!("\nResources: no update");
            }
        }

        match &update.async_op_update {
            Some(async_ops) => {
                println!("\nAsync operations:");
                println!("  new: {}", async_ops.new_async_ops.len());
                println!(
                    "  stats updated: {}",
                    async_ops.stats_update.len()
                );
                println!("  dropped events: {}", async_ops.dropped_events);
            }
            None => {
                println!("\nAsync operations: no update");
            }
        }

        let metadata_count = update
            .new_metadata
            .as_ref()
            .map_or(0, |metadata| metadata.metadata.len());

        println!("\nNew metadata: {metadata_count}");
    }
}