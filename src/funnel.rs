//! Wrapping channel
//!
//! A Funnel channel is one which wraps another channel, but implements
//! `Sender<F>` for all types `F` that implement `Into<T>`, where `T` is the
//! type expected by the wrapped channel. This is intended to be used in places
//! where a generic function expects some type implementing `Sender<T>`, but you
//! want that function to send messages on a channel with `Sender<F>`.

mod receiver;
mod sender;

use std::marker::PhantomData;

pub use receiver::*;
pub use sender::*;

use crate::{
	Receiver,
	Sender,
	err::SendError,
};

/// Wraps an existing channel, returning the sender/receiver halves.
///
/// The returned [FunnelSender] will behave the same as the [Sender] (`tx`) that
/// is passed to this function. Similarly, the returned [FunnelReceiver] will
/// behave the same as the [Receiver] (`rx`) that is passed to this function.
///
/// The [FunnelSender] will also implement `Sender<F>` for all types `F` that
/// implement `Into<T>`, where `T` is the type expected by `tx`.
pub fn channel<T, S, R>((tx, rx): (S, R)) -> (FunnelSender<T, S>, FunnelReceiver<T, R>)
where
	S: Sender<T>,
	R: Receiver<T>,
{
	(
		FunnelSender(tx, PhantomData),
		FunnelReceiver(rx, PhantomData),
	)
}

fn send<F, T, S>(sender: &S, value: F) -> Result<(), SendError<F>>
where
	S: Sender<T>,
	F: Into<T> + Copy,
{
	sender.send(value.into()).map_err(|_| SendError(value))
}
