fn main() {
    rf();
}

fn rf() {
    let contents = std::fs::read_to_string("test_file.txt").expect("could not read file");
    println!("\n{}", contents);
}
