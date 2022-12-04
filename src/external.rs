//! # Defining external examples
//!
//! You can define example crates with cargo.
//! These go into the `examples` directory by default (next to the `src` directory).
//!
//! I am going to use some special magic to include the code here:
//!
//! ```

#![doc=include_str!("../examples/erlang_the_movie.rs")]

//! ```
//!
//! You can run the example with
//!
//! ```shell
//! cargo run --example erlang_dialog
//! ```
//!
//! This feature is not strictly rustdoc-related,
//! but there is a new feature in rustdoc nightly, that will produce links to
//! examples when a function, that is used in an example, is documented.
//!
//! Advanced magic (nightly only): Scraped examples.
//!
//! ```shell
//! cargo doc -Zunstable-options -Zrustdoc-scrape-examples=examples
//! ```
//!
//! The results can be seen when you look at the [greet function](crate::greetings::greet) again.
//!
//! # That's a lot of syntax!
//!
//! Yes, we need to take time time to properly explain how to use [markdown](super::markdown).
