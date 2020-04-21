#[derive(Debug, PartialEq)]
pub struct BookWord {
    book_id: String,
    word_id: String,
}
impl BookWord {
    pub fn new(book_id: &str, word_id: &str) -> Self {
        BookWord {
            book_id: book_id.to_owned(),
            word_id: word_id.to_owned(),
        }
    }

    pub fn from_collection(ids: Vec<Vec<String>>) -> Vec<Self> {
        ids.iter()
            .map(|id_tuple| Self::new(&id_tuple[0], &id_tuple[1]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let book_word = BookWord::new("1", "2");
        let expected = BookWord {
            book_id: "1".to_string(),
            word_id: "2".to_string(),
        };

        assert_eq!(book_word, expected);
    }

    #[test]
    fn from_non_empty_collection() {
        let ids = vec![
            vec!["1".to_string(), "2".to_string()],
            vec!["2".to_string(), "1".to_string()],
        ];
        let collection = BookWord::from_collection(ids);

        let expected = vec![
            BookWord {
                book_id: "1".to_string(),
                word_id: "2".to_string(),
            },
            BookWord {
                book_id: "2".to_string(),
                word_id: "1".to_string(),
            },
        ];

        assert_eq!(collection, expected);
    }

    #[test]
    fn from_empty_collection() {
        let ids = vec![];
        let collection = BookWord::from_collection(ids);

        let expected = vec![];

        assert_eq!(collection, expected);
    }

    #[test]
    #[should_panic]
    fn from_incorrectly_sized_collection() {
        let ids = vec![
            vec!["1".to_string()],
            vec!["2".to_string(), "1".to_string()],
        ];
        BookWord::from_collection(ids);
    }
}
