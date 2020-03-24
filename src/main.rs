use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub mod user_book;
use user_book::model::UserBook;

fn main() -> std::io::Result<()> {
    read_users("foo.csv")
}

fn read_users(filename: &str) -> std::io::Result<()> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    let user_books = reader
        .lines()
        .skip(1)
        .map(|line| {
            line.ok()
                .unwrap()
                .split(',')
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .map(|ids| UserBook::new(ids[1].to_owned(), ids[0].to_owned()))
        .collect::<Vec<UserBook>>();

    println!("{:#?}", user_books);
    Ok(())
}
