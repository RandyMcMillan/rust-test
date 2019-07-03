fn collections() {
    let ints: Vec<i32> = Vec::new();
    let one_two_three = vec![1, 2, 3];

    // mutating vector
    let mut mutating_vector = Vec::new();

    mutating_vector.push(5);
    mutating_vector.push(6);
    mutating_vector.push(7);
    mutating_vector.push(8);


    // referencing elements in vector
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
