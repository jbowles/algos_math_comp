use std::io::{self, BufRead, BufReader, Read};
fn main() {
    print_by_line(io::stdin()).expect("error reading stdin");
}

fn print_by_line<T: Read>(reader: T) -> io::Result<()> {
    let buff = BufReader::new(reader);
    for line in buff.lines() {
        println!("{}", line?)
    }
    Ok(())
}
