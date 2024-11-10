use std::fs::read_dir;

pub fn readdir(path: String) {
    println!("readdir");

    let contents = read_dir(path).expect("path doesn't exist.");
    println!("{:#?}", contents);
}
