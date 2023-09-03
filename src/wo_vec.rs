/// Write only vector
/// allows take internal refs that live as long as the vec itself
/// keeping internal ref after vec destroyed is UB be careful
pub struct WoVec<T> {
    inner_vec: Vec<Box<T>>,
}

impl<T> WoVec<T> {
    pub fn new() -> Self {
        Self {
            inner_vec: Vec::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.inner_vec.push(Box::new(elem));
    }

    pub fn len(&self) -> usize {
        self.inner_vec.len()
    }

    /// mustny let the ref live longer than the vec
    pub unsafe fn get<'a>(&'a self, idx: usize) -> &'static T {
        std::mem::transmute::<&'a T, &'static T>(self.inner_vec[idx].as_ref())
    }

    /// static get cast makes any method to remove an element from the vector while still referenced ub
    pub unsafe fn get_inner(&self) -> &Vec<Box<T>> {
        &self.inner_vec
    }
}
