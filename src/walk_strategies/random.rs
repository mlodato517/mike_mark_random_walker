use super::Walker;
use crate::ids::Column;
use crate::metapath_map::MetapathMap;
use rand::Rng;

pub fn get_random_iter<'a, 'b, R: Rng>(
    col_1_id: u16,
    col: Column,
    rng: &'b mut R,
    map: &'a MetapathMap,
) -> Walker<'a, 'b, R> {
    Walker::Random(RandomWalkIter {
        current_id: col_1_id,
        col,
        rng,
        map,
    })
}

pub struct RandomWalkIter<'a, 'b, R: Rng> {
    current_id: u16,
    col: Column,
    rng: &'b mut R,
    map: &'a MetapathMap,
}

impl<'a, 'b, R: Rng> Iterator for RandomWalkIter<'a, 'b, R> {
    type Item = [[u8; 4]; 6];

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = [[0, 0, 0, 0]; 6];

        for empty_id in result.iter_mut() {
            let (next_id, next_col) = self.choose_random_id();
            let next_encoded_col_id = self.encoded_col_id_for(next_id, &next_col);
            *empty_id = next_encoded_col_id;

            self.current_id = next_id;
            self.col = next_col;
        }

        Some(result)
    }
}

impl<'a, 'b, R: Rng> RandomWalkIter<'a, 'b, R> {
    fn encoded_col_id_for(&mut self, id: u16, col: &Column) -> [u8; 4] {
        match col {
            Column::One => self.encoded_col_1_id(id),
            Column::Two => self.encoded_col_2_id(id),
            Column::Three => self.encoded_col_3_id(id),
            Column::Four => self.encoded_col_4_id(id),
        }
    }

    fn choose_random_id(&mut self) -> (u16, Column) {
        let random_col = self.choose_random_col();

        match (&self.col, random_col) {
            (Column::One, Column::Two) => {
                (self.col_2_id_for_col_1_id(self.current_id), Column::Two)
            }
            (Column::Two, Column::One) => {
                (self.col_1_id_for_col_2_id(self.current_id), Column::One)
            }
            (Column::Two, Column::Three) => {
                (self.col_3_id_for_col_2_id(self.current_id), Column::Three)
            }
            (Column::Three, Column::Two) => {
                (self.col_2_id_for_col_3_id(self.current_id), Column::Two)
            }
            (Column::Three, Column::Four) => {
                (self.col_4_id_for_col_3_id(self.current_id), Column::Four)
            }
            (Column::Four, Column::Three) => {
                (self.col_3_id_for_col_4_id(self.current_id), Column::Three)
            }
            _ => unreachable!(),
        }
    }

    fn choose_random_col(&mut self) -> Column {
        match self.col {
            Column::One => Column::Two,
            Column::Two => {
                let col_3_list = &self.map.get_col_2_to_3_map()[self.current_id as usize];
                let col_1_list = &self.map.get_col_2_to_1_map()[self.current_id as usize];

                let random = self.rng.gen_range(0, col_1_list.len() + col_3_list.len());

                if random > col_1_list.len() {
                    Column::Three
                } else {
                    Column::One
                }
            }
            Column::Three => {
                let col_4_list = &self.map.get_col_3_to_4_map()[self.current_id as usize];
                let col_2_list = &self.map.get_col_3_to_2_map()[self.current_id as usize];

                let random = self.rng.gen_range(0, col_2_list.len() + col_4_list.len());

                if random > col_2_list.len() {
                    Column::Four
                } else {
                    Column::Two
                }
            }
            Column::Four => Column::Three,
        }
    }

    fn col_2_id_for_col_1_id(&mut self, col_1_id: u16) -> u16 {
        let col_2_list = &self.map.get_col_1_to_2_map()[col_1_id as usize];
        col_2_list[self.rng.gen_range(0, col_2_list.len())]
    }

    fn col_3_id_for_col_2_id(&mut self, col_2_id: u16) -> u16 {
        let col_3_list = &self.map.get_col_2_to_3_map()[col_2_id as usize];
        col_3_list[self.rng.gen_range(0, col_3_list.len())]
    }

    fn col_4_id_for_col_3_id(&mut self, col_3_id: u16) -> u16 {
        let col_4_list = &self.map.get_col_3_to_4_map()[col_3_id as usize];
        col_4_list[self.rng.gen_range(0, col_4_list.len())]
    }

    fn col_3_id_for_col_4_id(&mut self, col_4_id: u16) -> u16 {
        let col_3_list = &self.map.get_col_4_to_3_map()[col_4_id as usize];
        col_3_list[self.rng.gen_range(0, col_3_list.len())]
    }

    fn col_2_id_for_col_3_id(&mut self, col_3_id: u16) -> u16 {
        let col_2_list = &self.map.get_col_3_to_2_map()[col_3_id as usize];
        col_2_list[self.rng.gen_range(0, col_2_list.len())]
    }

    fn col_1_id_for_col_2_id(&mut self, col_2_id: u16) -> u16 {
        let col_1_list = &self.map.get_col_2_to_1_map()[col_2_id as usize];
        col_1_list[self.rng.gen_range(0, col_1_list.len())]
    }

    fn encoded_col_1_id(&self, col_1_idx: u16) -> [u8; 4] {
        self.map.encoded_col_1_id(col_1_idx)
    }

    fn encoded_col_2_id(&self, col_2_idx: u16) -> [u8; 4] {
        self.map.encoded_col_2_id(col_2_idx)
    }

    fn encoded_col_3_id(&self, col_3_idx: u16) -> [u8; 4] {
        self.map.encoded_col_3_id(col_3_idx)
    }

    fn encoded_col_4_id(&self, col_4_idx: u16) -> [u8; 4] {
        self.map.encoded_col_4_id(col_4_idx)
    }
}
