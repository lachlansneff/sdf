use core::ops::{Index, IndexMut};

use glam::UVec2;

pub struct Grid<I> {
    width: usize,
    data: I,
}

impl<I> Grid<I> {
    pub fn new(width: usize, data: I) -> Self {
        Self { width, data }
    }
}

impl<I> Index<UVec2> for Grid<I>
where
    I: Index<usize>,
{
    type Output = <I as Index<usize>>::Output;

    fn index(&self, coords: UVec2) -> &Self::Output {
        &self.data[coords.y as usize * self.width + coords.x as usize]
    }
}

impl<I> IndexMut<UVec2> for Grid<I>
where
    I: IndexMut<usize>,
{
    fn index_mut(&mut self, coords: UVec2) -> &mut Self::Output {
        &mut self.data[coords.y as usize * self.width + coords.x as usize]
    }
}
