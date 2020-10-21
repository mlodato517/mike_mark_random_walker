use super::Walker;
use crate::ids::Column;
use crate::metapath_map::MetapathMap;
use rand::Rng;

pub fn get_metapath_iter<'a, 'b, R: Rng>(
    col_1_id: u16,
    _col: Column,
    rng: &'b mut R,
    map: &'a MetapathMap,
) -> Walker<'a, 'b, R> {
    Walker::Metapath(MetapathWalkIter { col_1_id, rng, map })
}

pub struct MetapathWalkIter<'a, 'b, R: Rng> {
    col_1_id: u16,
    rng: &'b mut R,
    map: &'a MetapathMap,
}

impl<'a, 'b, R: Rng> Iterator for MetapathWalkIter<'a, 'b, R> {
    type Item = [[u8; 4]; 6];

    fn next(&mut self) -> Option<Self::Item> {
        let col_2_id_1 = self.col_2_id_for_col_1_id(self.col_1_id);
        let col_3_id_1 = self.col_3_id_for_col_2_id(col_2_id_1);
        let col_4_id = self.col_4_id_for_col_3_id(col_3_id_1);
        let col_3_id_2 = self.col_3_id_for_col_4_id(col_4_id);
        let col_2_id_2 = self.col_2_id_for_col_3_id(col_3_id_2);
        let col_1_id_2 = self.col_1_id_for_col_2_id(col_2_id_2);

        self.col_1_id = col_1_id_2;

        Some([
            self.col_2_id(col_2_id_1, self.map),
            self.col_3_id(col_3_id_1, self.map),
            self.col_4_id(col_4_id, self.map),
            self.col_3_id(col_3_id_2, self.map),
            self.col_2_id(col_2_id_2, self.map),
            self.col_1_id(col_1_id_2, self.map),
        ])
    }
}

impl<'a, 'b, R: Rng> MetapathWalkIter<'a, 'b, R> {
    pub fn col_2_id_for_col_1_id(&mut self, col_1_id: u16) -> u16 {
        let col_2_list = &self.map.get_col_1_to_2_map()[col_1_id as usize];
        col_2_list[self.rng.gen_range(0, col_2_list.len())]
    }

    pub fn col_3_id_for_col_2_id(&mut self, col_2_id: u16) -> u16 {
        let col_3_list = &self.map.get_col_2_to_3_map()[col_2_id as usize];
        col_3_list[self.rng.gen_range(0, col_3_list.len())]
    }

    pub fn col_4_id_for_col_3_id(&mut self, col_3_id: u16) -> u16 {
        let col_4_list = &self.map.get_col_3_to_4_map()[col_3_id as usize];
        col_4_list[self.rng.gen_range(0, col_4_list.len())]
    }

    pub fn col_3_id_for_col_4_id(&mut self, col_4_id: u16) -> u16 {
        let col_3_list = &self.map.get_col_4_to_3_map()[col_4_id as usize];
        col_3_list[self.rng.gen_range(0, col_3_list.len())]
    }

    pub fn col_2_id_for_col_3_id(&mut self, col_3_id: u16) -> u16 {
        let col_2_list = &self.map.get_col_3_to_2_map()[col_3_id as usize];
        col_2_list[self.rng.gen_range(0, col_2_list.len())]
    }

    pub fn col_1_id_for_col_2_id(&mut self, col_2_id: u16) -> u16 {
        let col_1_list = &self.map.get_col_2_to_1_map()[col_2_id as usize];
        col_1_list[self.rng.gen_range(0, col_1_list.len())]
    }

    pub fn col_1_id(&self, col_1_idx: u16, map: &MetapathMap) -> [u8; 4] {
        map.encoded_col_1_id(col_1_idx)
    }

    pub fn col_2_id(&self, col_2_idx: u16, map: &MetapathMap) -> [u8; 4] {
        map.encoded_col_2_id(col_2_idx)
    }

    pub fn col_3_id(&self, col_3_idx: u16, map: &MetapathMap) -> [u8; 4] {
        map.encoded_col_3_id(col_3_idx)
    }

    pub fn col_4_id(&self, col_4_idx: u16, map: &MetapathMap) -> [u8; 4] {
        map.encoded_col_4_id(col_4_idx)
    }
}
