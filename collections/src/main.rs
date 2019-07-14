use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(2);
    v.push(1);

    for i in &v {
        println!("{}", i);
    }

    // mutate in place -> notice we're not using a pointer for v
    for i in &mut v {
        *i += 50; // dereference i
    }

    for i in &v {
        println!("{}", i);
    }

    let mut s = String::new(); // new String

    let g = String::from("initial contents"); // create String from string literal, UTF-8 encoded

    let data = "content";
    let data2 = data.to_string(); // convert to String from literal, equivalent to line 24

    s.push_str("more data");

    hash_maps();
    iterate_hash_maps();
    overwrite_map_value();
    insert_if_none();
    update_value();
}

fn hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 1);
    scores.insert(String::from("Red"), 5);

    // collate a hash map from existing vectors

    let teams = vec![String::from("Orange"), String::from("Yellow")];
    let initial_scores = vec![2, 6];

    // underscores are needed so Rust can infer the types
    // as collect() can take in many data types
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // hash maps gain ownership of owned values like String, but copies of values that implement `Copy`, like i32

    // fetch a value from a hash map via its key
    let selected_team = String::from("Orange");

    let selected_score = scores.get(&selected_team);

    println!("selected score for team {}: {:?}", selected_team, selected_score); // HashMap::get returns an Option<T>, so we can wrap it with a `match`
}

fn iterate_hash_maps() {
    // we can reuse this variable name because hash_map() releases ownership of `scores`
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Orange"), 10);
    scores.insert(String::from("Blue"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn overwrite_map_value() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Red"), 50); // we'll only see this value because Rust will overwrite values with the same key

    println!("{:?}", scores);
}

fn insert_if_none() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    // let orange_team = String::from("Orange");

    // scores.insert(orange_team, 25);
    scores.insert(String::from("Orange"), 25);

    // scores.entry(orange_team).or_insert(10); // will only insert if there is no value
    scores.entry(String::from("Orange")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(10);

    println!("inserting {:?}", scores);
}

fn update_value() {
    let text = "hello world wonderful world";

    let mut map: HashMap<String, i32> = HashMap::new();

    // checks to see if a word has been seen (added as a value)
    // if it has, increment it
    for word in text.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
