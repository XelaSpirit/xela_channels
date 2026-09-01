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
	F: Into<T> + Copy,
{
	fn send(&self, value: F) -> Result<(), SendError<F>>
	{
		send(&self.0, value)
	}
}

impl<F, T, S> AsyncSender<F> for FunnelSender<T, S>
where
	S: AsyncSender<T>,
	F: Into<T> + Copy,
{
}

impl<F, T, S> SyncSender<F> for FunnelSender<T, S>
where
	S: SyncSender<T>,
	F: Into<T> + Copy,
{
	fn try_send(&self, value: F) -> Result<(), TrySendError<F>>
	{
		self.0.try_send(value.into()).map_err(|err| {
			match err
			{
				| TrySendError::Full(_) => TrySendError::Full(value),
				| TrySendError::Disconnected(_) => TrySendError::Disconnected(value),
			}
		})
	}
}
