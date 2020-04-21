use std::io::prelude::*;

pub fn read<B: BufRead>(reader: B) -> std::io::Result<Vec<Vec<String>>> {
    Ok(reader
        .lines()
        .skip(1)
        .map(|line| line.unwrap())
        .map(|line| line.split(',').map(String::from).collect::<Vec<String>>())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_read() {
        let file = "user_id,book_id\n1,1\n1,2\n1,3\n2,1\n2,3\n".as_bytes();
        let reader = BufReader::new(file);
        let expectation = vec![
            vec!["1".to_string(), "1".to_string()],
            vec!["1".to_string(), "2".to_string()],
            vec!["1".to_string(), "3".to_string()],
            vec!["2".to_string(), "1".to_string()],
            vec!["2".to_string(), "3".to_string()],
        ];
        assert_eq!(read(reader).unwrap(), expectation);
    }
}
