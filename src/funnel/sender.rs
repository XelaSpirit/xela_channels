use std::marker::PhantomData;

use crate::{
	AsyncSender,
	Sender,
	SyncSender,
	err::{
		SendError,
		TrySendError,
	},
	funnel::send,
};

#[derive(Clone, Debug)]
pub struct FunnelSender<T, S>(pub(super) S, pub(super) PhantomData<T>)
where
	S: Sender<T>;

impl<F, T, S> Sender<F> for FunnelSender<T, S>
where
	S: Sender<T>,
	F: Into<T>,
	T: Into<F>,
{
	fn send(&self, value: F) -> Result<(), SendError<F>>
	{
		send(&self.0, value)
	}
}

impl<F, T, S> AsyncSender<F> for FunnelSender<T, S>
where
	S: AsyncSender<T>,
	F: Into<T>,
	T: Into<F>,
{
}

impl<F, T, S> SyncSender<F> for FunnelSender<T, S>
where
	S: SyncSender<T>,
	F: Into<T>,
	T: Into<F>,
{
	fn try_send(&self, value: F) -> Result<(), TrySendError<F>>
	{
		self.0.try_send(value.into()).map_err(|err| {
			match err
			{
				| TrySendError::Full(v) => TrySendError::Full(v.into()),
				| TrySendError::Disconnected(v) => TrySendError::Disconnected(v.into()),
			}
		})
	}
}
