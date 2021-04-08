
trait ConstDefault: Sized {
    const DEFAULT: Self;
}

impl ConstDefault for u32 {
    const DEFAULT: Self = 0;
}

#[repr(C)]
pub struct ArrayVec<T, const N: usize> {
    len: usize,
    arr: [T; N],
}

impl<T: Copy + ConstDefault, const N: usize> ArrayVec<T, N> {
    const INIT: [T; N] = [T::DEFAULT; N];

    pub fn new() -> Self {
        Self {
            len: 0,
            arr: Self::INIT,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn capacity(&self) -> usize {
        N
    }

    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    pub fn pop(&mut self) -> T {
        self.len -= 1;
        self.arr[self.len]
    }

    pub fn push(&mut self, item: T) {
        self.arr[self.len] = item;
        self.len += 1;
    }
}
