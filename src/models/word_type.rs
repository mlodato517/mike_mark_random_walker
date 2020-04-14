#[derive(Debug)]
pub struct WordType {
    type_id: String,
    word_id: String,
}
impl WordType {
    pub fn new(word_id: &str, type_id: &str) -> Self {
        WordType {
            type_id: type_id.to_owned(),
            word_id: word_id.to_owned(),
        }
    }

    pub fn from_collection(ids: Vec<Vec<String>>) -> Vec<Self> {
        ids.iter()
            .map(|id_tuple| Self::new(&id_tuple[0], &id_tuple[1]))
            .collect()
    }
}
