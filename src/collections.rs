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

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("enum debug list: {:?}", row);

    // hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Map: {:?}", scores);


    // vec to map via collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let zipped = teams.iter().zip(initial_scores.iter());

    let scores: HashMap<_, _> = zipped.collect();

    println!("Map: {:?}", scores);

    // I try vec<pair> to map
    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    let mappings: HashMap<_, _> = pairs.into_iter().collect();

    println!("Mapings: {:?}", mappings);

    // accessing
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("blue: {:?}", score);
    if let Some(scr) = score {
        println!("blue score: {}", scr);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}
