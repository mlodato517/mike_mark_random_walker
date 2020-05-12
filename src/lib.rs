mod metapath_map;
mod metapath_walk;

use metapath_map::MetapathMap;
use metapath_walk::walk;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

pub fn random_walk(
    col_1_2_filename: &str,
    col_2_3_filename: &str,
    col_3_4_filename: &str,
    output_filename: &str,
    lines_per_col_1_item: u8,
    walks_per_line: u8,
) {
    let metapath_map = MetapathMap::new(
        BufReader::new(File::open(col_1_2_filename).unwrap()),
        BufReader::new(File::open(col_2_3_filename).unwrap()),
        BufReader::new(File::open(col_3_4_filename).unwrap()),
    );

    let mut output_file =
        BufWriter::with_capacity(4 * 1024 * 1024, File::create(output_filename).unwrap());

    for col_1_id in metapath_map.col_1_ids() {
        let lines: Vec<Vec<u8>> = (0..lines_per_col_1_item)
            .into_par_iter()
            .map(|_| {
                let mut rng = rand::thread_rng();
                walk(walks_per_line, col_1_id as u16, &metapath_map, &mut rng)
            })
            .collect();
        for line in lines {
            output_file.write_all(&line).unwrap();
        }
    }

    output_file.flush().unwrap();
}
