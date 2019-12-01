use std::fs::File;
use std::io::prelude::*;

pub mod advent;

pub fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("file not found");
    let mut result = String::new();
    file.read_to_string(&mut result)
        .expect("something went wrong reading the file");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_a_test_file() {
        let content = read_file("Cargo.toml");
        assert_eq!(content.split("\n").next().unwrap(), "[package]");
    }

    #[test]
    #[should_panic]
    fn read_file_which_not_exist_should_panic() {
        read_file("dummy.txt");
    }
}
