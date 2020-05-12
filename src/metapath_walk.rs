use super::metapath_map::MetapathMap;
use rand::Rng;

pub fn walk<R>(walks_per_line: u8, col_1_id: u16, maps: &MetapathMap, mut rng: R) -> Vec<u8>
where
    R: Rng,
{
    let mut line = Vec::with_capacity(walks_per_line as usize * 6 * 4 + 4);
    line.extend_from_slice(&maps.col_1_id(col_1_id));

    let mut col_1_id = col_1_id;
    for _ in 0..walks_per_line {
        let col_2_id_1 = maps.random_col_2_id_for_col_1_id(col_1_id, &mut rng);
        let col_3_id_1 = maps.random_col_3_id_for_col_2_id(col_2_id_1, &mut rng);
        let col_4_id = maps.random_col_4_id_for_col_3_id(col_3_id_1, &mut rng);
        let col_3_id_2 = maps.random_col_3_id_for_col_4_id(col_4_id, &mut rng);
        let col_2_id_2 = maps.random_col_2_id_for_col_3_id(col_3_id_2, &mut rng);
        col_1_id = maps.random_col_1_id_for_col_2_id(col_2_id_2, &mut rng);

        line.push(b' ');
        line.extend_from_slice(&maps.col_2_id(col_2_id_1));
        line.push(b' ');
        line.extend_from_slice(&maps.col_3_id(col_3_id_1));
        line.push(b' ');
        line.extend_from_slice(&maps.col_4_id(col_4_id));
        line.push(b' ');
        line.extend_from_slice(&maps.col_3_id(col_3_id_2));
        line.push(b' ');
        line.extend_from_slice(&maps.col_2_id(col_2_id_2));
        line.push(b' ');
        line.extend_from_slice(&maps.col_1_id(col_1_id));
    }

    line.push(b'\n');
    line
}
