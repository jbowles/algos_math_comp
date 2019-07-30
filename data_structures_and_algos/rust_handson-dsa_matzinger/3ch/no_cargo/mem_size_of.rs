use std::mem;

//https://doc.rust-lang.org/std/mem/fn.size_of.html
fn main() {
    println!("Size of MyStruct: {:?}", mem::size_of::<MyStruct>());
    println!(
        "Size of MyStruct with 3 fields: {:?}",
        mem::size_of::<[MyStruct; 3]>()
    );

    println!("Size of OtherStruct: {:?}", mem::size_of::<OtherStruct>());
    println!(
        "Size of OtherStruct with 3 fields: {:?}",
        mem::size_of::<[OtherStruct; 3]>()
    );

    println!("Size of BigStruct: {:?}", mem::size_of::<BigStruct>());
    println!(
        "Size of BigStruct with 3 fields: {:?}",
        mem::size_of::<[BigStruct; 3]>()
    );
}

#[allow(dead_code)]
//3; 3*3 = 9
struct MyStruct {
    a: u8, //1byte
    b: u8, //1byte
    c: u8, //1byte
}

#[allow(dead_code)]
//14; 24*3 = 72
struct OtherStruct {
    a: f64, //8byte
    b: f64, //8byte
    c: f64, //8byte
}

#[allow(dead_code)]
//16; 16*3 = 144
struct BigStruct {
    a: i128, //16byte
    b: i128, //16byte
    c: i128, //16byte
}
