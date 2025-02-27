use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::shutdown_signal::ShutdownSignal;

/// Wrapped future that is automatically cancelled when a shutdown is triggered.
///
/// If the wrapped future completes before the shutdown is triggered,
/// the output of the original future is yielded as `Ok(value)`.
///
/// If the shutdown is triggered before the wrapped future completes,
/// the original future is dropped and the shutdown reason is yielded as `Err(shutdown_reason)`.
#[must_use = "futures must be polled to make progress"]
pub struct WrapCancel<T: Clone, F> {
	pub(crate) shutdown_signal: ShutdownSignal<T>,
	pub(crate) future: Result<F, T>,
}

impl<T: Clone, F: Future> Future for WrapCancel<T, F> {
	type Output = Result<F::Output, T>;

	#[inline]
	fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
		// SAFETY: We never move `future`, so we can not violate the requirements of `F`.
		let me = unsafe { self.get_unchecked_mut() };

		// SAFETY: We never move `future`, so we can not violate the requirements of `F`.
		// We do drop it, but that's fine.
		match &mut me.future {
			Err(e) => return Poll::Ready(Err(e.clone())),
			Ok(future) => {
				let future = unsafe { Pin::new_unchecked(future) };
				if let Poll::Ready(value) = future.poll(context) {
					return Poll::Ready(Ok(value));
				}
			},
		}

		// Otherwise check if the shutdown signal has been given.
		let shutdown = Pin::new(&mut me.shutdown_signal)
			.poll(context);
		match shutdown {
			Poll::Ready(reason) => {
				me.future = Err(reason.clone());
				Poll::Ready(Err(reason))
			},
			Poll::Pending => Poll::Pending,
		}
	}
}
