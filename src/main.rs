use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub mod models;
use models::user_book::UserBook;

fn main() -> std::io::Result<()> {
    let user_book_ids = read_csv("user_books.csv")?;
    let _user_books = UserBook::from_collection(user_book_ids);
    Ok(())
}

fn read_csv(filename: &str) -> std::io::Result<Vec<Vec<String>>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    Ok(reader
        .lines()
        .skip(1)
        .map(|line| line.unwrap())
        .map(|line| line.split(',').map(String::from).collect::<Vec<String>>())
        .collect())
}
