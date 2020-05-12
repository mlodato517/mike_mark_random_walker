extern crate rand;
extern crate structopt;

use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{BufWriter, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "generate_source_data",
    about = "Generates random relationship data for random walker."
)]
struct Opt {
    #[structopt(long = "num-col-1", help = "1-65535.", default_value = "5000")]
    num_col1: u16,

    #[structopt(long = "num-col-2", help = "1-65535.", default_value = "5000")]
    num_col2: u16,

    #[structopt(long = "num-col-3", help = "1-65535.", default_value = "500")]
    num_col3: u16,

    #[structopt(long = "num-col-4", help = "1-255.", default_value = "10")]
    num_col4: u8,

    #[structopt(long = "num-1-2-lines", help = "0-255.", default_value = "100")]
    num_col1_2: u8,

    #[structopt(long = "num-2-3-lines", help = "0-255.", default_value = "50")]
    num_col2_3: u8,

    #[structopt(long = "num-3-4-lines", help = "0-255.", default_value = "10")]
    num_col3_4: u8,

    #[structopt(
        long = "1-2-file",
        help = ".CSV with col1 <-> col2 relationships.",
        default_value = "col_1_2_relationships.csv"
    )]
    col_1_2_filename: String,

    #[structopt(
        long = "2-3-file",
        help = ".CSV with col2 <-> col3 relationships.",
        default_value = "col_2_3_relationships.csv"
    )]
    col_2_3_filename: String,

    #[structopt(
        long = "3-4-file",
        help = ".CSV with col3 <-> col4 relationships.",
        default_value = "col_3_4_relationships.csv"
    )]
    col_3_4_filename: String,
}
fn main() {
    let opt = Opt::from_args();

    if opt.num_col1 < 1 {
        panic!("num-col-1 must be at least 1")
    }
    if opt.num_col2 < 1 {
        panic!("num-col-2 must be at least 1")
    }
    if opt.num_col3 < 1 {
        panic!("num-col-3 must be at least 1")
    }
    if opt.num_col4 < 1 {
        panic!("num-col-4 must be at least 1")
    }

    let mut rng = thread_rng();
    let col_1_ids: Vec<String> = (0..opt.num_col1)
        .map(|_| rng.sample_iter(&Alphanumeric).take(10).collect())
        .collect();
    let col_2_ids: Vec<String> = (0..opt.num_col2)
        .map(|_| rng.sample_iter(&Alphanumeric).take(10).collect())
        .collect();
    let col_3_ids: Vec<String> = (0..opt.num_col3)
        .map(|_| rng.sample_iter(&Alphanumeric).take(10).collect())
        .collect();
    let col_4_ids: Vec<String> = (0..opt.num_col4)
        .map(|_| rng.sample_iter(&Alphanumeric).take(10).collect())
        .collect();

    let mut file = BufWriter::new(File::create(&opt.col_1_2_filename).unwrap());
    for id in col_1_ids {
        for _ in 0..opt.num_col1_2 {
            let line = format!("{},{}\n", id, col_2_ids.choose(&mut rng).unwrap());
            file.write_all(line.as_bytes()).unwrap();
        }
    }
    file.flush().unwrap();

    let mut file = BufWriter::new(File::create(&opt.col_2_3_filename).unwrap());
    for id in col_2_ids {
        for _ in 0..opt.num_col2_3 {
            let line = format!("{},{}\n", id, col_3_ids.choose(&mut rng).unwrap());
            file.write_all(line.as_bytes()).unwrap();
        }
    }
    file.flush().unwrap();

    let mut file = BufWriter::new(File::create(&opt.col_3_4_filename).unwrap());
    for id in col_3_ids {
        for _ in 0..opt.num_col3_4 {
            let line = format!("{},{}\n", id, col_4_ids.choose(&mut rng).unwrap());
            file.write_all(line.as_bytes()).unwrap();
        }
    }
    file.flush().unwrap();
}
