use std::{
	marker::PhantomData,
	ops::{
		Deref,
		DerefMut,
	},
	time::Duration,
};

use crate::{
	Receiver,
	err::{
		RecvError,
		RecvTimeoutError,
		TryRecvError,
	},
};

#[derive(Debug)]
pub struct FunnelReceiver<T, R>(pub(super) R, pub(super) PhantomData<T>)
where
	R: Receiver<T>;

impl<T, R> Receiver<T> for FunnelReceiver<T, R>
where
	R: Receiver<T>,
{
	fn recv(&self) -> Result<T, RecvError>
	{
		self.0.recv()
	}

	fn try_recv(&self) -> Result<T, TryRecvError>
	{
		self.0.try_recv()
	}

	fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError>
	{
		self.0.recv_timeout(timeout)
	}
}

impl<T, R> Deref for FunnelReceiver<T, R>
where
	R: Receiver<T>,
{
	type Target = R;

	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<T, R> DerefMut for FunnelReceiver<T, R>
where
	R: Receiver<T>,
{
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}
