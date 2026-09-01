//!# Xela Channels
//! 
//! `xela_channels` provides small, implementation-agnostic traits for message channels. They make it possible to write
//! generic functions that work with different channel implementations without depending on their concrete sender or
//! receiver types.
//! 
//! The library exports four main traits:
//! 
//! - [Sender] for sending messages.
//! - [SyncSender] for senders that also support non-blocking [try_send](SyncSender::try_send).
//! - [AsyncSender] for senders whose [send](Sender::send) operation does not block.
//! - [Receiver] for blocking, non-blocking, and timed receives.
//! 
//! Implementations are included for the channel types in `std::sync::mpsc`.
//! 
//! ## Example
//! 
//! The package is named `xela_channels`, while its Rust library name is `xch`:
//! 
//! **cargo.toml**
//! 
//! ```toml
//! [dependencies]
//! xela_channels = "0.1"
//! ```
//! 
//! **main.rs**
//! 
//! ```rust
//! use std::sync::mpsc;
//! use xch::{Receiver, Sender};
//! 
//! fn send_message<S: Sender<String>>(sender: &S, message: impl Into<String>) {
//! 	sender.send(message.into()).expect("receiver disconnected");
//! }
//! 
//! fn receive_message<R: Receiver<String>>(receiver: &R) -> String {
//! 	receiver.recv().expect("sender disconnected")
//! }
//! 
//! fn main() {
//! 	let (sender, receiver) = mpsc::channel();
//! 
//! 	send_message(&sender, "hello");
//! 	assert_eq!(receive_message(&receiver), "hello");
//! }
//! ```
//! 
//! ## Funnel channels
//! 
//! The [funnel] module wraps a channel so its sender can accept any `Copy` type that implements `Into<T>`, where `T` is the
//! wrapped channel's message type. This is useful when several generic producers send related message types into one
//! channel.
//! 
//! ```rust
//! use std::sync::mpsc;
//! use xch::{Sender, funnel};
//! 
//! fn main() {
//! 	let channel = mpsc::channel::<i64>();
//! 	let (sender, receiver) = funnel::channel(channel);
//! 
//! 	Sender::<i32>::send(&sender, 42).unwrap();
//! 	assert_eq!(receiver.recv().unwrap(), 42_i64);
//! }
//! ```
//! 

pub mod err;
pub mod funnel;
mod traits;

pub use traits::*;
