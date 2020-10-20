mod ids;
mod metapath_map;
mod walk_service;
mod walk_strategies;

use metapath_map::MetapathMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use walk_strategies::{metapath, random};

pub fn random_walk(
    col_1_2_filename: &str,
    col_2_3_filename: &str,
    col_3_4_filename: &str,
    output_filename: &str,
    lines_per_col_1_item: u8,
    walks_per_line: u8,
    strategy: &str,
) {
    let metapath_map = MetapathMap::new(
        BufReader::new(File::open(col_1_2_filename).unwrap()),
        BufReader::new(File::open(col_2_3_filename).unwrap()),
        BufReader::new(File::open(col_3_4_filename).unwrap()),
    );

    let walk_strategy = if strategy == "metapath" {
        metapath::get_metapath_iter as walk_strategies::Strategy<_>
    } else {
        random::get_random_iter as walk_strategies::Strategy<_>
    };

    let walk_service = walk_service::WalkService::new(&metapath_map, &walk_strategy);

    let mut output_file =
        BufWriter::with_capacity(4 * 1024 * 1024, File::create(output_filename).unwrap());

    for col_1_id in metapath_map.short_col_1_ids() {
        let lines = walk_service.get_lines_for_id(col_1_id, lines_per_col_1_item, walks_per_line);
        for line in lines {
            output_file.write(&line).unwrap();
        }
    }

    output_file.flush().unwrap();
}
