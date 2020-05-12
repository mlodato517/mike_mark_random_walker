extern crate structopt;

use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "random_walker",
    about = "Writes a random walk for metapath2vec"
)]
struct Opt {
    #[structopt(
        short = "n",
        long = "iterations-per-user",
        help = "Can be 1 to 255.",
        default_value = "128"
    )]
    iterations_per_user: u8,

    #[structopt(
        short = "w",
        long = "walks-per-line",
        help = "Can be 1 to 255.",
        default_value = "64"
    )]
    walks_per_line: u8,

    #[structopt(
        long = "1-2-file",
        help = "File with col1 <-> col2 relationship lines.csv'.",
        default_value = "col_1_2_relationships.csv"
    )]
    col_1_2_filename: String,

    #[structopt(
        long = "2-3-file",
        help = "File with col2 <-> col3 relationship lines.csv'.",
        default_value = "col_2_3_relationships.csv"
    )]
    col_2_3_filename: String,

    #[structopt(
        long = "3-4-file",
        help = "File with col3 <-> col4 relationship lines.csv'.",
        default_value = "col_3_4_relationships.csv"
    )]
    col_3_4_filename: String,

    #[structopt(
        short = "o",
        long = "output",
        help = "File to write to.",
        default_value = "output.txt"
    )]
    output_filename: String,
}

pub fn main() {
    let start = Instant::now();
    let opt = Opt::from_args();

    random_walker::random_walk(
        &opt.col_1_2_filename,
        &opt.col_2_3_filename,
        &opt.col_3_4_filename,
        &opt.output_filename,
        opt.iterations_per_user,
        opt.walks_per_line,
    );

    println!("Done! Took {}ms", start.elapsed().as_millis());
}
