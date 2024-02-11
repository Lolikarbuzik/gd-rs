# [gd-rs](https://crates.io/crates/gd-rs) &middot; ![Crates.io Version](https://img.shields.io/crates/v/gd-rs) ![Status](https://img.shields.io/badge/Status-NOT%20FINISHED-FA0000)

# The crate is in development
## If you have any questions dm me on discord @lolikarbuzik

# Docs
For documentation go to [docs](https://docs.rs/gd-rs/latest/gd_rs/)

# Getting started
Install the crate
```
cargo add gd-rs
```

Now in your file import `gd_rs`
```rust
use gd_rs::prelude::*;

let cclocallevels = CCLocalLevels::new(); // loads all the levels from the savefile
let my_level = cclocallevels.get(String::from("my level")); // returns option GDLocalLevel
println!("{my_level:?}");
```

I will start updating this crate soon

# Features in progress
- Levels into a plist so basically GDCCLocalLevels into their raw form
- Saving (so also 'encrypting') save files into their original form
