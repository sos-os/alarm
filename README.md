ALARM: Another Library for Allocating and Releasing Memory
==========================================================

[![Build Status](https://travis-ci.org/sos-os/alarm.svg?branch=master)](https://travis-ci.org/sos-os/alarm)

ALARM ~is~ _will be_ the new [SOS](https://github.com/hawkw/sos-kernel) memory allocator.

**NOTE:** ALARM is currently _very_ early in the development process and most functionality has yet to be implemented.

Crates
------

| Crate             | Description                                                       |
|-------------------|-------------------------------------------------------------------|
| `alarm-base`      | Base types and API definitions shared across ALARM allocators.    |
| `intruder-alarm`  | Intrusive collections library used for allocator data structures. |
| `slabby`          | Slab allocators composable on top of ALARM allocators.            |



Building ALARM
--------------

Building ALARM requires the [nightly Rust compiler](https://doc.rust-lang.org/book/first-edition/release-channels.html), which you can install using [`rustup`](https://www.rustup.rs/).

Although some components of ALARM may also be compatible with the stable Rust compiler, the core functionality of this library as a memory allocator introduces a hard dependency on the [`allocator_api`](https://github.com/rust-lang/rfcs/blob/master/text/1398-kinds-of-allocators.md) language feature, currently only available on the nightly compiler.

Continuous integration [builds](https://travis-ci.org/hawkw/alarm) of ALARM run against the _latest_ nightly compiler, so compatibility with older nightlies is not always assured.
