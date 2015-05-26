//! # Bidirectional Channels
//!
//! This library implements bidirectional channels in Rust.
//! The language has primitives to do this with the `channel`
//! function. `channel()` returns a `(sender, receiver)` pair
//! which can communicate with each other using message passing.
//! These channels are used to communicate through different
//! [tasks](http://doc.rust-lang.org/guide.html#tasks).
//!
//! A limitation of this is that only one can send messages,
//! and the other can only receive messages.
//! Using Rust's primitives we can extend this behaviour
//! to allow sending and receiving messages from both ends.
//! We call these ends Endpoints.
//!
//! ## Usage
//!
//! ```
//! extern crate bichannels;
//!
//! use std::thread;
//! use bichannels::{BiChannel};
//!
//! fn main() {
//!     let BiChannel{a, b} = BiChannel::new();
//!     let from_a = "hello";
//!     let from_b = 100;
//!
//!     let at = thread::spawn(move || {
//!         let _ = a.send(from_a);
//!         a.recv().unwrap()
//!     });
//!
//!     let bt = thread::spawn(move || {
//!         let _ = b.send(from_b);
//!         b.recv().unwrap()
//!     });
//!
//!     let to_a = at.join().unwrap();
//!     assert_eq!(to_a, from_b);
//!
//!     let to_b = bt.join().unwrap();
//!     assert_eq!(to_b, from_a);
//! }
//! ```

use std::sync::mpsc::{Sender, Receiver, channel, RecvError, SendError};

pub struct Endpoint<T1, T2> {
    sender: Sender<T1>,
    receiver: Receiver<T2>
}

// unsafe impl<T1: Send, T2: Send> Send for Endpoint<T1, T2> {}

impl<T1: Send, T2: Send> Endpoint<T1, T2> {
    pub fn send(&self, t: T1) -> Result<(), SendError<T1>>{
        self.sender.send(t)
    }
    pub fn recv(&self) -> Result<T2, RecvError> {
        self.receiver.recv()
    }
}

pub struct BiChannel<T1, T2> {
    pub a: Endpoint<T1, T2>,
    pub b: Endpoint<T2, T1>
}

impl<T1: Send, T2: Send> BiChannel<T1, T2> {
    pub fn new() -> BiChannel<T1, T2> {
        let (tx1, rx1) = channel();
        let (tx2, rx2) = channel();
        BiChannel {
            a: Endpoint{ sender: tx1, receiver: rx2 },
            b: Endpoint{ sender: tx2, receiver: rx1 },
        }
    }
}

#[test]
fn test1() {
    let BiChannel{a, b} = BiChannel::new();

    let ad = "1";
    let _ = a.send(ad.to_string());
    let ar = b.recv().unwrap();

    let bd = 1;
    let _ = b.send(bd);
    let br = a.recv().unwrap();

    assert_eq!(ad.to_string(), ar);
    assert_eq!(bd, br);
}
#[test]
fn test_thread() {
    use std::thread;
    let BiChannel{a, b} = BiChannel::new();
    let from_a = "hello";
    let from_b = 100;

    let at = thread::spawn(move || {
        let _ = a.send(from_a);
        a.recv().unwrap()
    });

    let bt = thread::spawn(move || {
        let _ = b.send(from_b);
        b.recv().unwrap()
    });

    let to_a = at.join().unwrap();
    assert_eq!(to_a, from_b);

    let to_b = bt.join().unwrap();
    assert_eq!(to_b, from_a);
}
