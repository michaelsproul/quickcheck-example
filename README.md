QuickCheck Lottery Scheduling Example
====

This repository contains a demo for using QuickCheck with Rust 1.0.

Don't worry if you don't know anything about [lottery scheduling][lot-sched],
only the basic data-structures are included here and they're nice and simple.

You can follow along with my [slides from the Rust 1.0 launch in Sydney][slides].

[lot-sched]: http://en.wikipedia.org/wiki/Lottery_scheduling
[slides]: http://michaelsproul.github.io/quickcheck-talk/

## Building

[Install the Rust 1.0 stable compiler][rust-install], then run:

```
$ cargo test
```

[rust-install]: http://www.rust-lang.org/

## Notes

* Check out all the uses of conditional compilation via `#[cfg(test)]`.
* Note tests in a separate module and file - `test.rs`.
* Note the implementation of `Arbitrary` for `Transaction` (in a separate module too!).
* Note the invariant checking function, `Lottery::is_valid`.

## Nonessential Notes

The code here exhibits a few differences from the code in the talk. There's a
macro - `transaction_test` - to abstract over running different application functions, and the
`bad_apply_transaction` is set up to be broken in different ways.

The use of `TEST_NUM_PLAYERS` simplifies the `Arbitrary` implementation, but is quite inelegant.
A more advanced approach would be to generate arbitrary `(Lottery, Vec<Transaction>)` tuples so that
the indicies in the transactions fit within the length of the Lottery's `tickets` vector.
