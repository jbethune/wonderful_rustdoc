//! # Showing code examples with code blocks
//!
//! `rustdoc` allows you to define Rust code examples by using triple backticks.
//!
//! ~~~text
//! ```
//! Rust code here
//! ```
//! ~~~
//!
//! It is also possible to define code in other languages.
//! In that case you need to add the code type after the opening backticks:
//!
//! ~~~text
//! ```python
//! Python code here
//! ```
//! ~~~
//!
//! When you run `cargo test`, the Rust code blocks will be compiled and executed.
//! Any errors will be reported.
//! This way your examples are always guaranteed to be up to date!
//!
//! And you get syntax highlighting for free!
//!
//! ```
//! pub struct Ferris {
//!   // zero sized type
//! }
//!
//! impl Ferris {
//!   pub fn new() -> Self {
//!     Self{}
//!   }
//!
//!   pub fn crab_dance(&self) {
//!      println!("Check your borrows, check them all");
//!      println!("Fearless concurrency stops the fall");
//!      println!("No reference must be left dangling");
//!      println!("Interop with C means no mangling");
//!   }
//! }
//! ```
//!
//! Let's look at more examples in the [greetings](super::greetings) module
//! and we will also learn about how to write documentation for functions.
