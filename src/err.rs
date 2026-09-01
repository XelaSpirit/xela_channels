use std::{
	error,
	fmt,
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SendError<T>(pub T);

pub enum TrySendError<T>
{
	Full(T),
	Disconnected(T),
}

pub struct RecvError;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TryRecvError
{
	Empty,
	Disconnected,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RecvTimeoutError
{
	Timeout,
	Disconnected,
}

impl<T> fmt::Debug for SendError<T>
{
	fn fmt(&self, frm: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		frm.debug_struct("SendError").finish_non_exhaustive()
	}
}

impl<T> fmt::Display for SendError<T>
{
	fn fmt(&self, frm: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		"sending on a closed channel".fmt(frm)
	}
}

impl<T> error::Error for SendError<T> {}

impl<T> fmt::Debug for TrySendError<T>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match *self
		{
			| TrySendError::Full(..) => "Full(..)".fmt(f),
			| TrySendError::Disconnected(..) => "Disconnected(..)".fmt(f),
		}
	}
}

impl<T> fmt::Display for TrySendError<T>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match *self
		{
			| TrySendError::Full(..) => "sending on a full channel".fmt(f),
			| TrySendError::Disconnected(..) => "sending on a closed channel".fmt(f),
		}
	}
}

impl<T> error::Error for TrySendError<T> {}

impl<T> From<SendError<T>> for TrySendError<T>
{
	fn from(err: SendError<T>) -> TrySendError<T>
	{
		match err
		{
			| SendError(t) => TrySendError::Disconnected(t),
		}
	}
}
