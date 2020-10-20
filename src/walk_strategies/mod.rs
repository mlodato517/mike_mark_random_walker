pub mod metapath;
pub mod random;

use crate::ids::Column;
use crate::MetapathMap;

use rand::Rng;

pub type WalkOfIds = [[u8; 4]; 6];

pub enum Walker<'a, 'b, R: Rng> {
    Metapath(metapath::MetapathWalkIter<'a, 'b, R>),
    Random(random::RandomWalkIter<'a, 'b, R>),
}

impl<'a, 'b, R: Rng> Iterator for Walker<'a, 'b, R> {
    type Item = WalkOfIds;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Walker::Metapath(m) => m.next(),
            Walker::Random(r) => r.next(),
        }
    }
}

// // The Strategy function returns a struct that wraps an iterator because
// // having a type alias that returns a struct directly that implements iterator
// // is not yet stable in rust.
// pub struct Walker {
//     iter: I,
// }
pub type Strategy<R> = for<'a, 'b> fn(
    col_1_id: u16,
    col: Column,
    rng: &'b mut R,
    map: &'a MetapathMap,
) -> Walker<'a, 'b, R>;
