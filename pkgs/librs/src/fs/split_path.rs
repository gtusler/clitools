use std::path::PathBuf;

pub fn split_path_buf(path: &PathBuf) -> Vec<String> {
    let path_str = path.to_str().expect("Unable to convert PathBuf to str");

    // there is definitely a better method of determining whether or not this is a windows path
    let separator = if path_str.contains("\\") { "\\" } else { "/" };

    let path_split_str: Vec<&str> = path_str.split(separator).collect();
    path_split_str.into_iter().map(|s| s.to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_splits_a_linux_path_in_two() {
        assert_eq!(
            split_path_buf(&PathBuf::from("something/deep")),
            vec![String::from("something"), String::from("deep")]
        );
    }

    #[test]
    fn it_splits_a_linux_path_in_three() {
        assert_eq!(
            split_path_buf(&PathBuf::from("something/really/deep")),
            vec![
                String::from("something"),
                String::from("really"),
                String::from("deep")
            ]
        );
    }

    #[test]
    fn it_splits_an_absolute_linux_path() {
        assert_eq!(
            split_path_buf(&PathBuf::from("/home/me/Documents")),
            vec![
                String::from(""),
                String::from("home"),
                String::from("me"),
                String::from("Documents")
            ]
        );
    }

    #[test]
    fn it_splits_a_windows_path_in_two() {
        assert_eq!(
            split_path_buf(&PathBuf::from("something\\deep")),
            vec![String::from("something"), String::from("deep")]
        );
    }

    #[test]
    fn it_splits_a_windows_path_in_three() {
        assert_eq!(
            split_path_buf(&PathBuf::from("something\\really\\deep")),
            vec![
                String::from("something"),
                String::from("really"),
                String::from("deep")
            ]
        );
    }

    #[test]
    fn it_splits_an_absolute_windows_path() {
        assert_eq!(
            split_path_buf(&PathBuf::from("C:\\Users\\me\\Documents")),
            vec![
                String::from("C:"),
                String::from("Users"),
                String::from("me"),
                String::from("Documents")
            ]
        );
    }
}
