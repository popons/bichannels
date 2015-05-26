# Bidirectional Channels

This library implements bidirectional channels in Rust.
The language has primitives to do this with the `channel`
function. `channel()` returns a `(sender, receiver)` pair
which can communicate with each other using message passing.
These channels are used to communicate through different
[tasks](http://doc.rust-lang.org/guide.html#tasks).

A limitation of this is that only one can send messages,
and the other can only receive messages.
Using Rust's primitives we can extend this behaviour
to allow sending and receiving messages from both ends.
We call these ends Endpoints.

## Usage

```rust
extern crate bichannels;

use std::thread;
use bichannels::{BiChannel};

fn main() {
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
```

