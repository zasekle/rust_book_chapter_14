
//! This type of documentation comment adds the comments to the object that contains the comment.
//! So in this case it will add it to the containing library.


/// This MUST be inside a library file in order for cargo test to run the code.
/// # Examples
///
/// ```
/// let arg = 5;
/// let result = rust_book_chapter_14::documented_function(arg);
///
/// assert_eq!(result, 6);
/// ```
pub fn documented_function(arg: isize) -> isize {
    arg + 1
}

//These will show the values on the front page of the documentation. This can allow the users of the
// library to more easily find them. This can also be used to export parts of another crate (or the
// entire crate) and show it on the front page.
pub use self::shapes::Corners;
pub use self::shapes::NoCorners;
pub use self::utils::remove_corners;

pub mod shapes {
    #[derive(Debug)]
    pub enum NoCorners {
        Circle,
        Oval,
    }

    pub enum Corners {
        Square,
        Rectangle,
    }
}

pub mod utils {
    use crate::shapes::*;

    /// Removes the corners from the shape and returns the respective type.
    pub fn remove_corners(shape: Corners) -> NoCorners {
        match shape {
            Corners::Square => NoCorners::Circle,
            Corners::Rectangle => NoCorners::Oval,
        }
    }
}
