//! # Free documentation hosting on docs.rs
//!
//! Let's say you have used
//!
//! ```shell
//! cargo publish
//! ```
//!
//! To push your crate to <https://crates.io>.
//! But people need to find your documentaion somehow!
//! Of course, there are search engines, but wouldn't it be much better if people could directly find your documentation?
//! This is what <https://docs.rs> is all about!
//!
//! Whenver a new crate is uploaded to <https://crates.io>,
//! the fine people at <https://docs.rs> will automatically download that crate's source code and generate the documentation.
//!
//! The documentation will be forever<sup>TM</sup> hosted on <https://docs.rs> completely free of charge!
//! This also means that if you want to look up documentation for a crate, you will most likely find it on <https://docs.rs>!
//!
//! ## Getting a crate's documentation locally
//!
//! When you use another crate in your project, `cargo doc` will also generate the documentation for your dependencies!
//! For example, this crate uses the [Grund](deutsche_bahn_delay_reasons::Grund) struct from the venerable `deutsche-bahn-delay-reasons` crate,
//! because I really like to write programs on the train.
//! If you look at the link,
//! you will see that the files are all hosted locally. But the same link on <https://docs.rs> will still point to <https://docs.rs>.
//!
//! ## Become a `rustdoc` power user
//!
//! There is [the official rustdoc book](https://doc.rust-lang.org/rustdoc/index.html),
//! which is also included in your `rustup` distribution.
//!
//! You can also learn a lot of secret tricks by clicking on the questionmark button next to the search field.
//!
//! # The end
//!
//! Go forth and write documentation!
//! You yourself will most likely benefit most from it, because your future self will be thankful for all the non-obvious things you have documented.
