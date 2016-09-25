/**
 * The result of an asynchronous Riak operation.
 * <p>
 * All Riak operations are asynchronous. It means when you execute an operation on
 * the cluster it will return immediately with no guarantee that the requested
 * operation has been completed at the end of the call. Instead, you will be returned
 * a {@code RiakFuture} instance which gives you the information about the result
 * or status of the operation.
 * </p>
 * <p>
 * A {@code RiakFuture} is either uncompleted or completed. When an operation begins, a new future
 * object is created. The new future is uncompleted initially - it is neither
 * succeeded, failed, nor canceled because the operation is not finished yet.
 * If the operation is finished either successfully, with failure, or by cancellation,
 * the future is marked as completed with more specific information, such as the
 * cause of the failure. Please note that even failure and cancellation belong to the completed state.
 * <pre>
 *                                      +---------------------------+
 *                                      | Completed successfully    |
 *                                      +---------------------------+
 *                                 +---->      isDone() = <b>true</b>      |
 * +--------------------------+    |    |   isSuccess() = <b>true</b>      |
 * |        Uncompleted       |    |    +===========================+
 * +--------------------------+    |    | Completed with failure    |
 * |      isDone() = <b>false</b>    |    |    +---------------------------+
 * |   isSuccess() = false    |----+---->      isDone() = <b>true</b>      |
 * | isCancelled() = false    |    |    |   isSuccess() = <b>false</b>     |
 * |       cause() = null     |    |    |       cause() = <b>non-null</b>  |
 * +--------------------------+    |    +===========================+
 *                                 |    | Completed by cancellation |
 *                                 |    +---------------------------+
 *                                 +---->      isDone() = <b>true</b>      |
 *                                      | isCancelled() = <b>true</b>      |
 *                                      +---------------------------+
 * </pre>
 * </p>
 * <p>
 * The typical use pattern is to call {@literal await()}, check {@literal isSuccess()}
 * then call {@literal getNow()} or {@literal cause()}</p>
 * @author Brian Roach <roach at basho dot com>
 * @param <V> the (response) return type
 * @param <T> The query info type
 * @since 2.0
 */
extern crate futures;
pub trait RiakFuture//<V, T>// extends Future<V>
{
    /**
     * Waits if necessary for at most the given time for the computation
     * to complete, and then retrieves its result, if available.
     * <p>
     * Note that the timeout value here is how long <b>you</b> are willing to wait
     * for this RiakFuture to complete. If you wish to set a timeout on the command
     * itself, use the timeout() method provided in the command's associated builder.
     * </p>
     * @param timeout the amount of time to wait before returning.
     * @param unit the unit of time.
     * @return the response, or null if not completed or failed.
     * @throws InterruptedException if the current thread was interrupted
     * while waiting.
     * @throws ExecutionException if the computation threw an
     * exception.
     * @throws TimeoutException if the wait timed out.
     */
    fn get(timeout: u64, unit: TimeUnit);
    fn isCancelled() -> bool;
    fn isDone()-> bool;
    /**
     * Waits for this RiakFuture to be completed for a set amount of time.
     * <p>Note that the timeout value here is how long <b>you</b> are willing to wait
     * for this RiakFuture to complete. Upon return you can check {@literal isDone()}
     * to see if the future has completed yet or not. The operation is still in
     * progress if that returns false.
     * </p>
     * <p>
     * If you wish to set a timeout on the command itself, use the timeout() method
     * provided in the command's associated builder.
     * </p>
     *
     * @param timeout the amount of time to wait before returning.
     * @param unit the unit of time.
     * @return {@code true} if the future completed and {@code false}
     *         if the waiting time elapsed before it completed
     * @throws InterruptedException if the current thread was interrupted
     * while waiting
     * @see #isDone()
     * @see #isSuccess()
     */
    fn await(timeout: u64, unit: TimeUnit)-> bool;
    /**
     * Return the result without blocking or throwing an exception.
     * If the future is not yet done or has failed this will return null.
     * As it is possible that a null value is used to mark the future as successful
     * you also need to check if the future is really done with {@link #isDone()}
     * and not rely on the returned null value.
     * @return The response, or {@literal null} if the future is not yet completed or failed.
     * @see #isDone()
     * @see #isSuccess()
     */
    fn getNow();
    /**
     * Determine if the operation completed successfully.
     * @return true if the operation succeeded, false if not yet completed or failed.
     */
    fn isSuccess() -> bool;
    /**
     * Returns information related to the operation performed.
     * <p>
     * Useful in async operations when you want to know what operation this
     * future refers to.
     * <p>
     * @return Information pertaining to the operation attached to this future.
     */
    fn getQueryInfo();
    /**
     * Add a listener to this RiakFuture.
     * @param listener a FiakFutureListener that will be notified when this RiakFuture completes.
     */
    //fn addListener(RiakFutureListener<V,T> listener);
    /**
     * Remove a listener from this RiakFuture.
     * @param listener The listener to remove.
     */
    //fn removeListener(RiakFutureListener<V,T> listener);
}
