# ruxt

An experimental implementation of Reactive Extensions in Rust


## Overview

There are quite few [Reactive Extensions](https://reactivex.io/) crates in Rust already.
Among the most popular is [rxrust](https://crates.io/crates/rxrust), but it depends on tokio runtime,
which is not necessary in many applications.
All I need is just an ergonomic way to build callback dependencies using Rust's powerful
functional style.

## Naming

I looked for a short and memorable name, but [rex](https://crates.io/crates/rex), [rax](https://crates.io/crates/rax), [rox](https://crates.io/crates/rox), [rux](https://crates.io/crates/rux), [rix](https://crates.io/crates/rix), [reax](https://crates.io/crates/reax) are all taken, so the next name I came up with was ruxt.
