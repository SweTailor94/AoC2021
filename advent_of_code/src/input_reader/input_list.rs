use std::fs::File;
use std::io::{BufReader, BufRead};

    pub fn get_vector_from_file<T, F>(file_name: &str, mut transform: F) -> Vec<T> where
        F: FnMut(&str) -> T {
        let mut all_of_them: Vec<T> = Vec::new();
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let val = transform(line.unwrap().as_str());
            all_of_them.push(val);
        }
        return all_of_them;
    }
