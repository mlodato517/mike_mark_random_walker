use std::fs::File;
use std::io::BufReader;

use crate::models::book_word::BookWord;
use crate::models::user_book::UserBook;
use crate::models::word_type::WordType;

use crate::utils::csv::read;

pub fn generate_random_walk() -> std::io::Result<()> {
    let f = File::open("user_books.csv")?;
    let user_book_ids = read(BufReader::new(f))?;
    let _user_books = UserBook::from_collection(user_book_ids);

    let f = File::open("book_words.csv")?;
    let book_word_ids = read(BufReader::new(f))?;
    let _book_words = BookWord::from_collection(book_word_ids);

    let f = File::open("word_types.csv")?;
    let word_type_ids = read(BufReader::new(f))?;
    let _x = WordType::from_collection(word_type_ids);
    println!("{:#?}, {:#?}, {:#?}", _user_books, _book_words, _x);
    Ok(())
}
