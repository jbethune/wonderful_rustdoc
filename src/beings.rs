//! # Various living beings
//!
//! Flora and Fauna can be found here.


/// A human being
///
/// Warning: Humans have feelings!
///
/// You can document individual fields by using `///` doc comments.
/// But these doc-comments have to come *before* the field, not after it.
///
/// Wrong example:
///
/// ```
/// /// docstring for the entire struct
/// pub struct Human {
///   pub name: String, /// No, this is NOT where the documentation should go
///   pub age: usize, // because the above docstring would be linked to the age field
/// }
/// ```
///
/// Correct example:
/// ```
/// /// docstring for the entire struct
/// pub struct Human {
///   /// This is the right spot for documenting the name field
///   pub name: String,
///   /// and this is the docstring for the age field
///   pub age: usize,
/// }
/// ```
///
/// Now check out the actual source for this module, to see how it is done.
/// After that we want to look at [external examples](super::external).
pub struct Human {
    /// What someone is called
    pub name: String,
    /// How many times they have been around the sun
    pub age: usize,
}
