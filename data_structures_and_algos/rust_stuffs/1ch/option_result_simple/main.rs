use std::io::{Error, ErrorKind};
use std::path::Path;

fn main() {
    let params = vec![2,9];
    for n in params {
        println!("SEARCH for paramter: {}", n);

        if let Some(result) = find(n, vec![1,2,3,4,5]) {
            println!("Match? {} for {}", result, n);
        }

        if let Some(result) = findother(n, vec![1,2,3,4,5]) {
            println!("Index {}, Value {} for parameter {}", result.0, result.1, n);
        }

        println!("Find {}? {}", n, otherfind(n, vec![1,2,3,4,5]));
        println!("...");
        println!(" ");
    }

    let filestrs = vec!["testfile.txt", "/tmp/not/a/file","../door_struct/main.rs"];
    for f in filestrs {
        println!("LOOK for file: {}", f);

        match read_file(f.to_string()) {
            Ok(result) => println!("Found it '{}'", result),
            Err(err) => println!("{}", err)
        }

        println!("___");
        println!(" ");
    }
}

fn find(needle: u16, haystack: Vec<u16>) -> Option<u16> {
    for n in haystack.iter() {
        if needle == *n {
            return Some(*n)
        }
    }
    Some(0)
}

fn otherfind(needle: u16, haystack: Vec<u16>) -> bool {
    for n in haystack.iter() {
        if needle == *n {
            return true
        }
    }
    return false
}

fn findother(needle: u16, haystack: Vec<u16>) -> Option<(usize,u16)> {
    for (i,n) in haystack.iter().enumerate() {
        if needle == *n {
            return Some((i,*n))
        }
    }
    Some((0,0))
}

fn read_file(fname: String) -> Result<String, Error> {
    if Path::new(&fname).exists() {
        return Ok(fname)
    }
    Err(Error::new(ErrorKind::Other, "File Does Not Exist"))
}
