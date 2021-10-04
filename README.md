# async-shutdown

Runtime agnostic one-stop solution for graceful shutdown in asynchronous code.

This crate addresses two separate but related problems regarding graceful shutdown:
* You have to be able to stop running futures when a shutdown signal is given.
* You have to be able to wait for futures to finish potential clean-up.

Both issues are handled by the [`Shutdown`] struct.

## Stopping running futures
To stop running futures, you can get a future to wait for the shutdown signal with [`Shutdown::wait_shutdown_triggered()`].
In this case you must write your async code to react to the shutdown signal appropriately.

Alternatively, you can wrap a future to be cancelled (by being dropped) when the shutdown starts with [`Shutdown::wrap_cancel()`].
This doesn't require the wrapped future to know anything about the shutdown signal,
but it also doesn't allow the future to run custom shutdown code.

To trigger the shutdown signal, simply call [`Shutdown::shutdown()`].

## Waiting for futures to complete.
If you have futures that run custom shutdown code (as opposed to just dropping the futures),
you will want to wait for that cleanup code to finish.
You can do that with [`Shutdown::wait_shutdown_complete()`].
That function returns a future that only completes when the shutdown is "completed".

You must also prevent the shutdown from completing too early by calling [`Shutdown::delay_shutdown_token()`] or [`Shutdown::wrap_wait()`].
Note that this can only be done before a shutdown has completed.
If the shutdown is already complete those functions will return an error.

The [`Shutdown::delay_shutdown_token()`] function gives you a [`DelayShutdownToken`] which prevents the shutdown from completing.
To allow the shutdown to finish, simply drop the token.
Alternatively, [`Shutdown::wrap_wait()`] wraps an existing future,
and will prevent the shutdown from completing until the future either completes or is dropped.

You can also use a token to wrap a future with [`DelayShutdownToken::wrap_wait()`].
This has the advantage that it can never fail:
the fact that you have a token means the shutdown has not finished yet.

## Automatically triggering shutdowns
You can also cause a shutdown when vital tasks or futures stop.
Call [`Shutdown::vital_token()`] to obtain a "vital" token.
When a vital token is dropped, a shutdown is triggered.

You can also wrap a future to cause a shutdown on completion using [`Shutdown::wrap_vital()`] or [`VitalToken::wrap_vital()`].
This can be used as a convenient way to terminate all asynchronous tasks when a vital task stops.

## Futures versus Tasks
Be careful when using `JoinHandles` as if they're a regular future.
Depending on your async runtime, when you drop a `JoinHandle` this doesn't necessarily cause the task to stop.
It may simply detach the join handle from the task, meaning that your task is still running.
If you're not careful, this could still cause data loss on shutdown.
As a rule of thumb, you should usually wrap futures *before* you spawn them on a new task.

[`Shutdown`]: https://docs.rs/async-shutdown/latest/async_shutdown/struct.Shutdown.html
[`Shutdown::wait_shutdown_triggered()`]: https://docs.rs/async-shutdown/latest/async_shutdown/struct.Shutdown.html#method.wait_shutdown_triggered
[`Shutdown::wrap_cancel()`]:  https://docs.rs/async-shutdown/latest/async_shutdown/struct.Shutdown.html#method.wrap_cancel
[`Shutdown::shutdown()`]:  https://docs.rs/async-shutdown/latest/async_shutdown/struct.Shutdown.html#method.shutdown
[`Shutdown::wait_shutdown_complete()`]:  https://docs.rs/async-shutdown/latest/async_shutdown/struct.Shutdown.html#method.wait_shutdown_complete
[`Shutdown::delay_shutdown_token()`]:  https://docs.rs/async-shutdown/latest/async_shutdown/struct.Shutdown.html#method.delay_shutdown_token
[`Shutdown::wrap_wait()`]:  https://docs.rs/async-shutdown/latest/async_shutdown/struct.Shutdown.html#method.wrap_wait
[`DelayShutdownToken`]: https://docs.rs/async-shutdown/latest/async_shutdown/struct.DelayShutdownToken.html
[`DelayShutdownToken::wrap_wait()`]: https://docs.rs/async-shutdown/latest/async_shutdown/struct.DelayShutdownToken.html#method.wrap_wait
[`Shutdown::vital_token()`]:  https://docs.rs/async-shutdown/latest/async_shutdown/struct.Shutdown.html#method.vital_token
[`Shutdown::wrap_vital()`]:  https://docs.rs/async-shutdown/latest/async_shutdown/struct.Shutdown.html#method.wrap_vital
[`VitalToken::wrap_vital()`]: https://docs.rs/async-shutdown/latest/async_shutdown/struct.VitalToken.html#method.wrap_vital
