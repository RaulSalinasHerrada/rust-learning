//! # My Crate Art
//!  
//!  `My Crate` is a collection of utilities to performs
//! certain calculations

pub mod kinds {
    pub enum PrimaryColor{
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor{
        Orange,
        Green,
        Purple
    }
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equeals amounts to create
    /// a secondary color
    ///
    pub fn mix(
        c1:PrimaryColor,
        c2: PrimaryColor,
    ) -> SecondaryColor {
        // --snip--
        // ANCHOR_END: here
        SecondaryColor::Orange
        // ANCHOR: here
    }
}


/// Adds one to the given number
///
/// # Examples
///
///```
/// let arg = 5;
/// assert_eq!(6, my_crate::add_one(5));
//```
pub fn add_one(x: i32) -> i32{
    x + 1
}

