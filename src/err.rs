//! Error types returned by channel send and receive operations.
//!
//! These implementation-agnostic errors allow generic channel code to handle
//! failures consistently regardless of the underlying channel type.

use std::{
	error,
	fmt,
	fmt::Debug,
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SendError<T>(pub T);

impl<T> fmt::Debug for SendError<T>
where
	T: Debug,
{
	fn fmt(&self, frm: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		frm.debug_tuple("SendError").field(&self.0).finish()
	}
}

impl<T> fmt::Display for SendError<T>
{
	fn fmt(&self, frm: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		fmt::Display::fmt("sending on a closed channel", frm)
	}
}

impl<T> error::Error for SendError<T> where T: Debug {}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TrySendError<T>
{
	Full(T),
	Disconnected(T),
}

impl<T> fmt::Debug for TrySendError<T>
where
	T: Debug,
{
	fn fmt(&self, frm: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			| TrySendError::Full(v) => frm.debug_tuple("Full").field(&v).finish(),
			| TrySendError::Disconnected(v) => frm.debug_tuple("Disconnected").field(&v).finish(),
		}
	}
}

impl<T> fmt::Display for TrySendError<T>
{
	fn fmt(&self, frm: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			| TrySendError::Full(..) => fmt::Display::fmt("sending on a full channel", frm),
			| TrySendError::Disconnected(..) => fmt::Display::fmt("sending on a closed channel", frm),
		}
	}
}

impl<T> error::Error for TrySendError<T> where T: Debug {}

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

#[derive(Debug)]
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
