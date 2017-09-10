use Size;
use position::Position;

pub trait Indexed<T>: Size {
    /// This retrieves the element at the specified position in the matrix, or None if it's out of
    /// bounds
    fn get(&self, position: (usize,usize)) -> Option<&T>;
}
