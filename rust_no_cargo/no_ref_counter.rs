fn main(){
    no_ref_counter();
    no_ref_counter_clone();
    no_ref_counter_loop();
    ref_counter_with_clone();
}


use std::rc::Rc;
#[derive(Debug)]
struct FileNameRc {
    name: Rc<String>,
    ext: Rc<String>
}

fn ref_counter_with_clone() {
    let name = Rc::new(String::from("main"));
    let ext = Rc::new(String::from("rs"));

    for _ in 0..3 {
        println!("RcCLONE: {:?}", FileNameRc {
            name: name.clone(),
            ext: ext.clone(),
        });
    }
}

#[derive(Debug)]
struct FileName {
    name: String,
    ext: String
}

// see no_ref_counter_broken()
fn no_ref_counter() {
    let name = String::from("main");
    let ext = String::from("rs");
    //for loop is a scope, for any number, even 1
    /*
    for _ in 0..3 { // borrowed under more than 1 scope is illeagal; use of moved value: `name`
        println!("{:?}", FileName {
            name: name,
            ext: ext,
        });
    }
    */

        println!("NOCLONE: {:?}", FileName {
            name: name,
            ext: ext,
        });
}

fn no_ref_counter_clone() {
    let name = String::from("main");
    let ext = String::from("rs");

    for _ in 0..3 {
        println!("CLONE: {:?}", FileName {
            name: name.clone(),
            ext: ext.clone(),
        });
    }
}



//not even unsafe{} lets you do this!!
fn no_ref_counter_loop() {
    for _ in 0..3 { // borrowed under more than one scope is illegal;
        let name = String::from("main");
        let ext = String::from("rs");
        println!("LOOPALLOC: {:?}", FileName {
            name: name,
            ext: ext,
        });
    }
}


/*
//not even unsafe{} lets you do this!!
fn no_ref_counter_broken() {
    let name = String::from("main");
    let ext = String::from("rs");
    //for loop is a scope, for any number, even 1
    for _ in 0..1 { // borrowed under more than one scope is illegal;
        println!("{:?}", FileName {
            name: name, //use of moved value: `name`
            ext: ext, //use of moved value: `ext`
        });
    }
}
*/

