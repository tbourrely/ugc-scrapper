use clokwerk::{AsyncScheduler, Job, TimeUnits};
use std::time::Duration;
use clokwerk::Interval::Tuesday;
use sqlx::{Pool, Postgres};
use crate::jobs::scrap_ugc_movies_task::{retrieve_movies_from_ugc, test};

pub fn start_scheduler(db: &Pool<Postgres>) {
    // Create scheduler with specific timezone
    let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

    // Add some tasks to it
    scheduler.every(Tuesday).at("03:00:00").run(|| retrieve_movies_from_ugc(db));
    scheduler.every(5.seconds()).run(|| test());

    // Or spawn a task to run it forever
    tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
}