
/// The trait for all Riak Commands.
/// <p>
/// All the commands the {@link RiakClient} can execute extend this class.
pub trait RiakCommand<T> {
    fn execute_async(cluster: &RiakCluster,
                     timeout: u64,
                     unit: TimeUnit,
                     cmd: RiakFuture<T>)
                     -> Result<String, String>;
    // fn abstract RiakFuture<T, S> executeAsync(cluster: &RiakCluster);
}
