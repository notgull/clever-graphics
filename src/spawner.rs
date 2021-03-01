// MIT/Apache2 License

use orphan_crippler::Receiver;

/// A trait representing a smart pointer to a thread pool that blocking tasks can be spawned on.
pub trait Spawner {
    fn spawn_blocking<T: Send + Sync + 'static, F: FnOnce() -> T + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> Receiver<T>;
}
