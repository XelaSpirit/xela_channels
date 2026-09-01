use std::{
	sync::mpsc,
	time::Duration,
};

use crate::err::{
	RecvError,
	RecvTimeoutError,
	SendError,
	TryRecvError,
	TrySendError,
};

/// Trait describing the sending half of a message channel. Messages can be sent
/// through the channel using [send](Sender::send).
///
/// It is heavily encouraged that types implementing this trait also implement
/// one of [SyncSender] or [AsyncSender] depending on whether
/// [send](Sender::send) may block the calling thread.
pub trait Sender<T>
{
	/// Attempts to send a value on the channel, returning it back if it could
	/// not be sent.
	///
	/// Returns [Ok] when it is determined that the other end of the channel has
	/// not hung up already, [Err] when the corresponding
	/// [Receiver(s)](Receiver) have already been deallocated.
	///
	/// Note that a return value of [Err] means that the data will never be
	/// received, but a return value of [Ok] does *not* mean that the data
	/// *will* be received. It is possible for the corresponding
	/// [Receiver(s)](Receiver) to hang up immediately after this function
	/// returns [Ok].
	///
	/// Implementations of this function may or may not block the current
	/// thread. If it is necessary to know the blocking behavior of the
	/// implementing type, use [SyncSender] or [AsyncSender].
	fn send(&self, value: T) -> Result<(), SendError<T>>;
}

/// Trait describing the sending half of a synchronous message channel.
///
/// Messages can be sent through the channel using [send](Sender::send) or
/// [try_send](SyncSender::try_send).
///
/// [send](Sender::send) may block the calling thread. See the documentation for
/// the implementing type for more details.
pub trait SyncSender<T>: Sender<T>
{
	/// Attempts to send a value on this channel without blocking.
	///
	/// Unlike [send](Sender::send), this method will return immediately if the
	/// channel is unable to send the message. See the documentation for the
	/// implementing type for the particular conditions where this happens.
	///
	/// See [send](Sender::send) for notes about whether the receiver will
	/// receive the data or not if this function is successful.
	fn try_send(&self, value: T) -> Result<(), TrySendError<T>>;
}

/// Trait describing the sending half of an asynchronous message channel.
///
/// This trait does not include any new functions, but is instead used by an
/// implementing type to indicate that [send](Sender::send) will never block the
/// calling thread.
pub trait AsyncSender<T>: Sender<T> {}

/// Trait describing the receiving half of a message channel. Messages sent to
/// the channel can be retrieved with any of the functions under this trait.
pub trait Receiver<T>
{
	/// Attempts to wait for a value on the receiver, returning an error if the
	/// corresponding channel has hung up.
	///
	/// This function will always block the current thread if there is no data
	/// available, and it's possible for more data to be sent (at least one
	/// sender still exists). Once a message is sent to the corresponding
	/// [Sender] (or [SyncSender]), this receiver will wake up and return that
	/// message.
	///
	/// If the corresponding [Sender(s)](Sender) have disconnected, or they
	/// disconnect while this call is blocking, this call will wake up and
	/// return [Err] to indicate that no more messages can ever be received on
	/// the channel.
	fn recv(&self) -> Result<T, RecvError>;

	/// Attempts to return a pending value on the receiver without blocking.
	///
	/// This method will never block the caller in order to wait for data to
	/// become available. Instead, this will always return immediately with a
	/// possible option of pending data on the channel.
	fn try_recv(&self) -> Result<T, TryRecvError>;

	/// Attempts to wait for a value on this receiver, returning an error if the
	/// corresponding channel has hung up, or if it waits more `timeout`.
	///
	/// This function will always block the current thread if there is no data
	/// available, and it's possible for more data to be sent (at least one
	/// sender still exists). Once a message is sent to the channel, the
	/// receiver will wake up and return that message.
	///
	/// If the corresponding [Sender(s)](Sender) have disconnected, or they
	/// disconnect while this call is blocking, this call will wake up and
	/// return [Err] to indicate that no more messages can ever be received on
	/// this channel.
	fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError>;
}

impl<T> Sender<T> for mpsc::Sender<T>
{
	fn send(&self, value: T) -> Result<(), SendError<T>>
	{
		mpsc::Sender::send(self, value).map_err(|err| SendError(err.0))
	}
}

impl<T> AsyncSender<T> for mpsc::Sender<T> {}

impl<T> Sender<T> for mpsc::SyncSender<T>
{
	fn send(&self, value: T) -> Result<(), SendError<T>>
	{
		mpsc::SyncSender::send(self, value).map_err(|err| SendError(err.0))
	}
}

impl<T> SyncSender<T> for mpsc::SyncSender<T>
{
	fn try_send(&self, value: T) -> Result<(), TrySendError<T>>
	{
		mpsc::SyncSender::try_send(self, value).map_err(|err| {
			match err
			{
				| mpsc::TrySendError::Full(value) => TrySendError::Full(value),
				| mpsc::TrySendError::Disconnected(err) => TrySendError::Disconnected(err),
			}
		})
	}
}

impl<T> Receiver<T> for mpsc::Receiver<T>
{
	fn recv(&self) -> Result<T, RecvError>
	{
		mpsc::Receiver::recv(self).map_err(|_| RecvError)
	}

	fn try_recv(&self) -> Result<T, TryRecvError>
	{
		mpsc::Receiver::try_recv(self).map_err(|err| {
			match err
			{
				| mpsc::TryRecvError::Empty => TryRecvError::Disconnected,
				| mpsc::TryRecvError::Disconnected => TryRecvError::Disconnected,
			}
		})
	}

	fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError>
	{
		mpsc::Receiver::recv_timeout(self, timeout).map_err(|err| {
			match err
			{
				| mpsc::RecvTimeoutError::Disconnected => RecvTimeoutError::Disconnected,
				| mpsc::RecvTimeoutError::Timeout => RecvTimeoutError::Timeout,
			}
		})
	}
}
