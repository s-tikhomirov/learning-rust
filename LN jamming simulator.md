Note and questions while (re-)implementing my LN jamming simulator in Rust (originally written in Python).

Comparison of custom `enum`s: is `#[derive(PartialEq)]` enough? What exactly does it do? When do I have to implement my own comparison operators and how do I do it?

How to write tests?

How to structure a project, how to import files, namespaces something something.

Apply constraints to scalar types? https://www.reddit.com/r/rust/comments/swiqw7/whats_the_right_way_to_apply_constraints_to/

Related: constraint the range of values of a primitive type (that's not so obvious from the first glance - no dependent types in Rust?): https://stackoverflow.com/questions/27673674/is-there-a-way-to-create-a-data-type-that-only-accepts-a-range-of-values
(I wanted to use it for a probability type that would only accept values from 0 to 1; I could also import it from some probability crate, but I don't yet know how to do that either. Leaving as is, this probability stuff isn't used much in my code anyway.)

How to use an external crate? Open its page on crates.io and do as stated (example of priority queue):
```
Run the following Cargo command in your project directory:
cargo add priority-queue

Or add the following line to your Cargo.toml:
priority-queue = "1.3.1"
```

## Changes compared to Python implementation

### Only linear fee functions
In Python, we stored separately: the two coefficients for the linear fee function in ChannelInDirection instance (base fee and fee rate), _and_ a function object, which was the generic fee function partially applied. Its likely possible to do it in Rust, but for simplicity for now we only store the Fee object, which contains the two coefficients. This means that the Rust implementation doesn't support fee functions that are not in the form of base fee plus amount times fee rate. This may be extended later via modifying the Fee type.

### Reset slots with no num_slots parameters
Let's try to be more immutable! Nothing awful would happen if I only allow resetting the slots priority queue with the corresponding number of slots that is already part of the ChannelInDirection object. Thinking about it, it is probably true that I only stored `num_slots` separately because there was no way in Python to get the maximal queue size after it is created. Is it possible in Rust?

## How to construct IDs?
In the simulator, we have various types of objects that have their Ids, namely: Payment, Channel, Node. I want to construct an ID as a random string. But I don't understand how to do the following:

A `Payment` is a recursive data structure. The outermost payment contains an id, an amount, a fee (simplifying here a bit), and the _downstream_ `Payment`, which is another instance of the same `struct`. This continues until the inner-most `Payment`, which has `downstream_payment: None`. The logic is: each of the forwarding nodes in the path "unwraps" one level of Payment and forwards the inner payment to the downstream node. The last node sees that there is no inner payment, this means, it is the receiver.

The issue is: I want all nested payments to share the same ID. This is needed (mostly?) for debug purposes? It also reflects the reality of Lightning where channel updates that are part of the same payment are united by the same payment hash (that I call payment ID). I tried implementing a `PaymentId(pub String)`, but I can't copy (clone?) the id into the downstream payment when I construct the nested `Payment` struct, because:

> `dp` has type `Box<Payment>`, which does not implement the `Copy` trait

I use `Box` because otherwise Rust can't allocate memory for a recursive data structure. (Not that I now precisely what `Box` does though.)

Possible solutions:
	1. understand why a String can't be cloned and somehow copy it anyway
	2. convert a String into a string literal: after all, IDs are constant strings that don't change. I don't understand how to do it. I know that a `String` can be implicitly cast to `&str`, but can it _become_ a `&str`?
	3. get rid of Ids altogether: if it's only needed for debugging, do we really really need it?

Other IDs, like channel IDs and node IDs, shouldn't have to get copied.

A related question: is `String` (heap-allocated) the right type for a constant ID anyway?

For now (2023-05-28), I remove id from Payment, let's return to this issue later. See also: [nanoid](https://docs.rs/nanoid/latest/nanoid/index.html) (returns `String` too). So, people do use Strings for IDs, so this is not something completely stupid...

NB: in the Python implementation, `Event`s also have ID for ordering: it's possible to have two events where all fields have the same values. Is this a problem though if two equal elements are in a queue? We'll find out.

## References in struct, Box, and cloning
Question: should we use references in structs? Answer: perhaps, but this requires lifetimes, which I haven't learned about yet. Generally speaking, why not, for example, allocate `PaymentResult::SUCCESS` on the heap and let all payments with this result reference the one allocated instance? This doesn't contradict anything, does it? We still can perform all payment resolution logic looking up the result "by reference".

It seems that I `clone()` too much during `Payment` creation, which may or may not be what I need. Generally, a `Payment` is a read-only structure: it gets constructed and then destructed (resolved). We never modify fields of a `Payment` instance after it is created. This invites us to use immutable references, right? Can't do it until I understand the relationship between: `Box`, recursive data structures, references, cloning.