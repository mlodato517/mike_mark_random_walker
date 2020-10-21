use super::ids::{COL_1_PREFIX, COL_2_PREFIX, COL_3_PREFIX, COL_4_PREFIX};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

type Lines = Vec<Vec<String>>;
pub type Map = Vec<Vec<u16>>;
pub type NonPrefixedId = [u8; 4];

pub struct MetapathMap {
    base64_cache: Vec<NonPrefixedId>,
    col_1_to_2_map: Map,
    col_2_to_3_map: Map,
    col_3_to_4_map: Map,
    col_4_to_3_map: Map,
    col_3_to_2_map: Map,
    col_2_to_1_map: Map,
}

impl MetapathMap {
    pub fn new<R>(col_1_2_file: R, col_2_3_file: R, col_3_4_file: R) -> Self
    where
        R: BufRead,
    {
        let (col_1_2_lines, col_2_3_lines, col_3_4_lines) =
            Self::get_filtered_lines(col_1_2_file, col_2_3_file, col_3_4_file);

        let (
            col_1_to_2_map,
            col_2_to_1_map,
            col_2_to_3_map,
            col_3_to_2_map,
            col_3_to_4_map,
            col_4_to_3_map,
        ) = Self::maps_from_lines(col_1_2_lines, col_2_3_lines, col_3_4_lines);

        let lengths = [
            col_1_to_2_map.len(),
            col_2_to_3_map.len(),
            col_3_to_4_map.len(),
            col_4_to_3_map.len(),
        ];
        let longest_len = lengths.iter().max().unwrap();

        let base64_cache: Vec<_> = (0..*longest_len).map(|n| Self::encode(n as u16)).collect();

        MetapathMap {
            base64_cache,
            col_1_to_2_map,
            col_2_to_3_map,
            col_3_to_4_map,
            col_4_to_3_map,
            col_3_to_2_map,
            col_2_to_1_map,
        }
    }

    pub fn short_col_1_ids(&self) -> impl Iterator<Item = u16> {
        (0..self.col_1_to_2_map.len()).map(|n| n as u16)
    }

    pub fn get_col_1_to_2_map(&self) -> &Map {
        &self.col_1_to_2_map
    }

    pub fn get_col_2_to_3_map(&self) -> &Map {
        &self.col_2_to_3_map
    }

    pub fn get_col_3_to_4_map(&self) -> &Map {
        &self.col_3_to_4_map
    }

    pub fn get_col_4_to_3_map(&self) -> &Map {
        &self.col_4_to_3_map
    }

    pub fn get_col_3_to_2_map(&self) -> &Map {
        &self.col_3_to_2_map
    }

    pub fn get_col_2_to_1_map(&self) -> &Map {
        &self.col_2_to_1_map
    }

    pub fn encoded_col_1_id(&self, col_1_idx: u16) -> [u8; 4] {
        let mut non_prefixed_id = self.base64_cache[col_1_idx as usize];
        non_prefixed_id[0] = COL_1_PREFIX;

        let prefixed_id = non_prefixed_id;
        prefixed_id
    }

    pub fn encoded_col_2_id(&self, col_2_idx: u16) -> [u8; 4] {
        let mut non_prefixed_id = self.base64_cache[col_2_idx as usize];
        non_prefixed_id[0] = COL_2_PREFIX;

        let prefixed_id = non_prefixed_id;
        prefixed_id
    }

    pub fn encoded_col_3_id(&self, col_3_idx: u16) -> [u8; 4] {
        let mut non_prefixed_id = self.base64_cache[col_3_idx as usize];
        non_prefixed_id[0] = COL_3_PREFIX;

        let prefixed_id = non_prefixed_id;
        prefixed_id
    }

    pub fn encoded_col_4_id(&self, col_4_idx: u16) -> [u8; 4] {
        let mut non_prefixed_id = self.base64_cache[col_4_idx as usize];
        non_prefixed_id[0] = COL_4_PREFIX;

        let prefixed_id = non_prefixed_id;
        prefixed_id
    }

    fn get_split_lines<R>(file: R) -> Lines
    where
        R: BufRead,
    {
        file.lines()
            .map(|line| line.unwrap())
            .map(|line| line.split(',').map(String::from).collect())
            .collect()
    }

    fn get_filtered_lines<R: BufRead>(
        col_1_2_file: R,
        col_2_3_file: R,
        col_3_4_file: R,
    ) -> (Lines, Lines, Lines) {
        let col_1_2_lines = Self::get_split_lines(col_1_2_file);
        let col_2_ids: HashSet<&str> = col_1_2_lines.iter().map(|ids| ids[1].as_str()).collect();

        let col_2_3_lines: Vec<Vec<String>> = Self::get_split_lines(col_2_3_file)
            .into_iter()
            .filter(|ids| col_2_ids.contains(ids[0].as_str()))
            .collect();
        let col_3_ids: HashSet<&str> = col_2_3_lines.iter().map(|ids| ids[1].as_str()).collect();

        let col_3_4_lines: Vec<Vec<String>> = Self::get_split_lines(col_3_4_file)
            .into_iter()
            .filter(|ids| col_3_ids.contains(ids[0].as_str()))
            .collect();
        let col_3_ids: HashSet<&str> = col_3_4_lines.iter().map(|ids| ids[0].as_str()).collect();

        let col_2_3_lines: Vec<Vec<String>> = col_2_3_lines
            .into_iter()
            .filter(|ids| col_3_ids.contains(ids[1].as_str()))
            .map(|ids| ids.into_iter().map(String::from).collect())
            .collect();
        let col_2_ids: HashSet<&str> = col_2_3_lines.iter().map(|ids| ids[0].as_str()).collect();

        let col_1_2_lines: Vec<Vec<String>> = col_1_2_lines
            .into_iter()
            .filter(|ids| col_2_ids.contains(ids[1].as_str()))
            .collect();

        (col_1_2_lines, col_2_3_lines, col_3_4_lines)
    }

    fn maps_from_lines(
        col_1_2_lines: Lines,
        col_2_3_lines: Lines,
        col_3_4_lines: Lines,
    ) -> (Map, Map, Map, Map, Map, Map) {
        let mut col_1_map = HashMap::new();
        let mut col_2_map = HashMap::new();
        for ids in &col_1_2_lines {
            let new_idx = col_1_map.len();
            col_1_map.entry(ids[0].to_owned()).or_insert(new_idx);

            let new_idx = col_2_map.len();
            col_2_map.entry(ids[1].to_owned()).or_insert(new_idx);
        }

        let mut col_1_to_2_map = vec![Vec::new(); col_1_map.len()];
        let mut col_2_to_1_map = vec![Vec::new(); col_2_map.len()];
        for ids in &col_1_2_lines {
            let col_1_entry = col_1_map[&ids[0]];
            let col_2_entry = col_2_map[&ids[1]];
            col_1_to_2_map[col_1_entry].push(col_2_entry as u16);
            col_2_to_1_map[col_2_entry].push(col_1_entry as u16);
        }

        let mut col_3_map = HashMap::new();
        for ids in &col_2_3_lines {
            let new_idx = col_3_map.len();
            col_3_map.entry(ids[1].to_owned()).or_insert(new_idx);
        }

        let mut col_2_to_3_map = vec![Vec::new(); col_2_map.len()];
        let mut col_3_to_2_map = vec![Vec::new(); col_3_map.len()];
        for ids in &col_2_3_lines {
            let col_2_entry = col_2_map[&ids[0]];
            let col_3_entry = col_3_map[&ids[1]];
            col_2_to_3_map[col_2_entry].push(col_3_entry as u16);
            col_3_to_2_map[col_3_entry].push(col_2_entry as u16);
        }

        let mut col_4_map = HashMap::new();
        for ids in &col_3_4_lines {
            let new_idx = col_4_map.len();
            col_4_map.entry(ids[1].to_owned()).or_insert(new_idx);
        }

        let mut col_3_to_4_map = vec![Vec::new(); col_3_map.len()];
        let mut col_4_to_3_map = vec![Vec::new(); col_4_map.len()];
        for ids in &col_3_4_lines {
            let col_3_entry = col_3_map[&ids[0]];
            let col_4_entry = col_4_map[&ids[1]];
            col_3_to_4_map[col_3_entry].push(col_4_entry as u16);
            col_4_to_3_map[col_4_entry].push(col_3_entry as u16);
        }

        (
            col_1_to_2_map,
            col_2_to_1_map,
            col_2_to_3_map,
            col_3_to_2_map,
            col_3_to_4_map,
            col_4_to_3_map,
        )
    }

    fn encode(n: u16) -> NonPrefixedId {
        let mut buf = [0; 4];
        base64::encode_config_slice(n.to_ne_bytes(), base64::STANDARD_NO_PAD, &mut buf[1..]);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_new_1_2_file() {
        let col_1_2_file = String::from("1_0,2_0\n1_0,2_1\n1_1,2_0\n1_1,2_2");
        let col_1_2_file = BufReader::new(col_1_2_file.as_bytes());

        let expected_1_2_map = vec![vec![0, 1], vec![0, 2]];
        let expected_2_1_map = vec![vec![0, 1], vec![0], vec![1]];

        let col_2_3_file = String::from("2_0,3_0\n2_0,3_1\n2_1,3_0\n2_2,3_2");
        let col_2_3_file = BufReader::new(col_2_3_file.as_bytes());

        let col_3_4_file = String::from("3_0,4_0\n3_1,4_0\n3_1,4_1\n3_2,4_1");
        let col_3_4_file = BufReader::new(col_3_4_file.as_bytes());

        let maps = MetapathMap::new(col_1_2_file, col_2_3_file, col_3_4_file);

        assert_eq!(maps.col_1_to_2_map, expected_1_2_map);
        assert_eq!(maps.col_2_to_1_map, expected_2_1_map);
    }

    #[test]
    fn test_new_2_3_file() {
        let col_1_2_file = String::from("1_0,2_0\n1_0,2_1\n1_1,2_0\n1_1,2_2");
        let col_1_2_file = BufReader::new(col_1_2_file.as_bytes());

        let col_2_3_file = String::from("2_0,3_0\n2_0,3_1\n2_1,3_0\n2_2,3_2");
        let col_2_3_file = BufReader::new(col_2_3_file.as_bytes());

        let expected_2_3_map = vec![vec![0, 1], vec![0], vec![2]];
        let expected_3_2_map = vec![vec![0, 1], vec![0], vec![2]];

        let col_3_4_file = String::from("3_0,4_0\n3_1,4_0\n3_1,4_1\n3_2,4_1");
        let col_3_4_file = BufReader::new(col_3_4_file.as_bytes());

        let maps = MetapathMap::new(col_1_2_file, col_2_3_file, col_3_4_file);

        assert_eq!(maps.col_2_to_3_map, expected_2_3_map);
        assert_eq!(maps.col_3_to_2_map, expected_3_2_map);
    }

    #[test]
    fn test_new_3_4_file() {
        let col_1_2_file = String::from("1_0,2_0\n1_0,2_1\n1_1,2_0\n1_1,2_2");
        let col_1_2_file = BufReader::new(col_1_2_file.as_bytes());

        let col_2_3_file = String::from("2_0,3_0\n2_0,3_1\n2_1,3_0\n2_2,3_2");
        let col_2_3_file = BufReader::new(col_2_3_file.as_bytes());

        let col_3_4_file = String::from("3_0,4_0\n3_1,4_0\n3_1,4_1\n3_2,4_1");
        let col_3_4_file = BufReader::new(col_3_4_file.as_bytes());

        let expected_3_4_map = vec![vec![0], vec![0, 1], vec![1]];
        let expected_4_3_map = vec![vec![0, 1], vec![1, 2]];

        let maps = MetapathMap::new(col_1_2_file, col_2_3_file, col_3_4_file);

        assert_eq!(maps.col_3_to_4_map, expected_3_4_map);
        assert_eq!(maps.col_4_to_3_map, expected_4_3_map);
    }

    #[test]
    fn test_get_maps_filtering_col_2_no_col_3() {
        let col_1_2_file = String::from("1_0,2_0\n1_0,2_1");
        let col_1_2_file = BufReader::new(col_1_2_file.as_bytes());

        // Note that the col_2 ID "2_1" has no col_3 associations
        let col_2_3_file = String::from("2_0,3_0");
        let col_2_3_file = BufReader::new(col_2_3_file.as_bytes());

        let col_3_4_file = String::from("3_0,4_0");
        let col_3_4_file = BufReader::new(col_3_4_file.as_bytes());

        let maps = MetapathMap::new(col_1_2_file, col_2_3_file, col_3_4_file);

        assert_eq!(maps.col_1_to_2_map, vec![vec![0]]);
        assert_eq!(maps.col_2_to_1_map, vec![vec![0]]);
        assert_eq!(maps.col_2_to_3_map, vec![vec![0]]);
        assert_eq!(maps.col_3_to_2_map, vec![vec![0]]);
        assert_eq!(maps.col_3_to_4_map, vec![vec![0]]);
        assert_eq!(maps.col_4_to_3_map, vec![vec![0]]);
    }

    #[test]
    fn test_get_maps_filtering_col_3_no_col_4() {
        let col_1_2_file = String::from("1_0,2_0");
        let col_1_2_file = BufReader::new(col_1_2_file.as_bytes());

        let col_2_3_file = String::from("2_0,3_0\n2_0,3_1");
        let col_2_3_file = BufReader::new(col_2_3_file.as_bytes());

        // Notice that the col_3 ID "3_1" has no col_4 associations
        let col_3_4_file = String::from("3_0,4_0");
        let col_3_4_file = BufReader::new(col_3_4_file.as_bytes());

        let maps = MetapathMap::new(col_1_2_file, col_2_3_file, col_3_4_file);

        assert_eq!(maps.col_1_to_2_map, vec![vec![0]]);
        assert_eq!(maps.col_2_to_1_map, vec![vec![0]]);
        assert_eq!(maps.col_2_to_3_map, vec![vec![0]]);
        assert_eq!(maps.col_3_to_2_map, vec![vec![0]]);
        assert_eq!(maps.col_3_to_4_map, vec![vec![0]]);
        assert_eq!(maps.col_4_to_3_map, vec![vec![0]]);
    }

    #[test]
    fn test_get_maps_filtering_col_3_no_col_2() {
        let col_1_2_file = String::from("1_0,2_0");
        let col_1_2_file = BufReader::new(col_1_2_file.as_bytes());

        let col_2_3_file = String::from("2_0,3_0");
        let col_2_3_file = BufReader::new(col_2_3_file.as_bytes());

        // Notice that the col_3 ID "3_1" has no col_2 associations
        let col_3_4_file = String::from("3_0,4_0\n3_1,4_0");
        let col_3_4_file = BufReader::new(col_3_4_file.as_bytes());

        let maps = MetapathMap::new(col_1_2_file, col_2_3_file, col_3_4_file);

        assert_eq!(maps.col_1_to_2_map, vec![vec![0]]);
        assert_eq!(maps.col_2_to_1_map, vec![vec![0]]);
        assert_eq!(maps.col_2_to_3_map, vec![vec![0]]);
        assert_eq!(maps.col_3_to_2_map, vec![vec![0]]);
        assert_eq!(maps.col_3_to_4_map, vec![vec![0]]);
        assert_eq!(maps.col_4_to_3_map, vec![vec![0]]);
    }

    #[test]
    fn test_get_maps_filtering_col_2_no_col_1() {
        let col_1_2_file = String::from("1_0,2_0");
        let col_1_2_file = BufReader::new(col_1_2_file.as_bytes());

        // Notice that the col_2 ID "2_1" has no col_1 associations
        let col_2_3_file = String::from("2_0,3_0\n2_1,3_1");
        let col_2_3_file = BufReader::new(col_2_3_file.as_bytes());

        let col_3_4_file = String::from("3_0,4_0\n3_1,4_0");
        let col_3_4_file = BufReader::new(col_3_4_file.as_bytes());

        let maps = MetapathMap::new(col_1_2_file, col_2_3_file, col_3_4_file);

        assert_eq!(maps.col_1_to_2_map, vec![vec![0]]);
        assert_eq!(maps.col_2_to_1_map, vec![vec![0]]);
        assert_eq!(maps.col_2_to_3_map, vec![vec![0]]);
        assert_eq!(maps.col_3_to_2_map, vec![vec![0]]);
        assert_eq!(maps.col_3_to_4_map, vec![vec![0]]);
        assert_eq!(maps.col_4_to_3_map, vec![vec![0]]);
    }
}
