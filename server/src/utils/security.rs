use std::{
    future::Future,
    pin::Pin,
    time::{Duration, Instant},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub async fn ensure_execution_time<'a, T, F>(duration: Duration, func: F) -> T
where
    F: FnOnce() -> Pin<Box<dyn Future<Output = T> + Send + 'a>>,
{
    let now = Instant::now();
    let result = func().await;

    let duration = duration - now.elapsed();

    if duration > Duration::from_millis(0) {
        tokio::time::sleep(duration).await;
    }

    result
}

pub fn random_string(n: usize) -> String {
    let rng = thread_rng();
    String::from_utf8(rng.sample_iter(&Alphanumeric).take(n).collect::<Vec<u8>>()).unwrap()
}
