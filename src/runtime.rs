use tokio::task;
use std::time::Duration;

pub async fn delay(seconds: u64) {
    tokio::time::sleep(Duration::from_secs(seconds)).await;
}

pub fn spawn<F>(fut: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    task::spawn(fut);
}
