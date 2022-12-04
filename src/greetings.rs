//! Create nice greatings.
//! 
//! Greetings are very important for [humans][Human]. They signal the beginning of an interaction and enable
//! further conversation that can lead to productive results. Greetings are therefore a highly
//! important topic.
//!
//! # More `rustdoc` features!
//!
//! If you want to document a specific item, you use the `///` comments
//! (that is: 3 instead of 2 slashes) to document the *next* item.
//! Check out the `greet` function below
//! and look at the source to see how the documentation is written.

use crate::beings::Human;

/// Generate a friendly greeting message
///
/// This function currently only supports english greetings.
/// Future versions might be expanded to change this.
///
/// *(reminder: check out the source code of the documentation)*
///
/// # Code Examples
///
/// ## A lovely conversation between cryptography enthusiasts
/// ```
/// use wonderful_rustdoc::greetings::greet;
/// use wonderful_rustdoc::beings::Human;
///
/// let alice = Human{name: "Alice".into(), age: 20};
/// let bob = Human{name: "Bob".into(), age: 20};
/// let eve = Human{name: "Eve".into(), age: 7};
///
/// println!("It is a sunny summer day in Cryptoville.");
/// println!("{} sits in the bush and listens to the conversation that {} and {} have.", eve.name, alice.name, bob.name);
/// println!("Alice: {}", greet(&bob));
/// println!("Bob: {}", greet(&alice));
/// ```
///
/// # Reading on
///
/// `rustdoc` automatically links type arguments in the documentation.
/// Have a look at the `Human` type, by clicking on the link in the API line!
pub fn greet(who: &Human) -> String {
    format!("Hello {}!", who.name)
}
