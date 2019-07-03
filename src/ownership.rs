fn ownership() {
    let mut str = String::from("hello");

    reference(&str);
    mutable(&mut str);

    test(str);

    let mut mutable = String::from("hi");
    let immutable = mutable.clone();
//    immutable.push_str("!");
}

fn test(str: String) {
    println!("test{:?}", str);
}

fn reference(str: &String) {
    println!("ref {:?}", str);
}

fn mutable(str: &mut String) {
    str.push_str("!");
    println!("mut: {:?}", str);
}
