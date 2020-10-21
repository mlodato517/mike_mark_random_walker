use crate::ids::Column;
use crate::metapath_map::MetapathMap;
use crate::walk_strategies::Strategy;
use rayon::prelude::*;

pub struct WalkService<'a> {
    map: &'a MetapathMap,
    strategy: &'a Strategy<rand::rngs::ThreadRng>,
}

const SPACE: u8 = b' ';
impl<'a> WalkService<'a> {
    pub fn new(map: &'a MetapathMap, strategy: &'a Strategy<rand::rngs::ThreadRng>) -> Self {
        WalkService { map, strategy }
    }

    pub fn get_lines_for_id(
        &self,
        col_1_id: u16,
        lines_per_col_1_item: u8,
        walks_per_line: u8,
    ) -> Vec<Vec<u8>> {
        (0..lines_per_col_1_item)
            .into_par_iter()
            .map(|_| {
                let mut rng = rand::thread_rng();
                let iter = (self.strategy)(col_1_id, Column::One, &mut rng, self.map);

                let line_parts = iter
                    .take(walks_per_line as usize)
                    .map(|bytes| bytes.join(&SPACE))
                    .collect::<Vec<Vec<u8>>>();

                let mut start = vec![self.map.encoded_col_1_id(col_1_id).to_vec()];
                start.extend_from_slice(&line_parts[..]);

                let mut line = start.join(&SPACE);
                line.push(b'\n');

                line
            })
            .collect()
    }
}
