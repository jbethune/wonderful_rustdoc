//! # What is `rustdoc`?
//!
//! `rustdoc` is a tool to convert Rust code into documentation.
//! It can be used standalone, but most likely you want to invoke it from `cargo`:
//!
//! ```shell
//! cargo doc --open
//! ```
//!
//! The above command generates the documentation and opens it in a browser.
//!
//! # Why `rustdoc`?
//!
//! Most public code needs documentation on how to use it.
//! While `rustdoc` can generate a lot of documentation by itself,
//! it is still necessary for humans to provide additional information.
//!
//! # How to use `rustdoc`?
//!
//! You add special documentation comments in your source code.
//! If you want to write documentation for a module,
//! you insert documentation lines that start with `//!` to describe the module file itself.
//! You can check out how it is done for *this* module,
//! by clicking on the link with the text "source" at the top right of the page.
//!
//! # Tell me more
//!
//! Have a look at the [codeblocks] module.

pub mod beings;
pub mod codeblocks;
pub mod external;
pub mod greetings;
pub mod hosting;
pub mod markdown;
