//! # Chap 14
//! `Chap 14's crate` is a collection of utilities to make performing certain
//! calculations more convenient.


/// This is a documentation comment.
/// it can use *markdown*, isn't that **cool**?
/// # Examples
/// ```
/// let arg = 5;
/// let answer = chap_14::add_three(arg);
/// assert_eq!(8, answer);
/// ```
/// View this in the browser with `cargo doc --open`.
/// Similar to how you can open the rust book with `rustup doc --book`.
pub fn add_three(a: i32) -> i32 {
    a + 3
}

pub mod kinds;
pub use kinds::PrimaryColor;
