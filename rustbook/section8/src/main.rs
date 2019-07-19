#[derive(Debug)]
enum FilledInfo {
    Age(i32),
    Name(String),
    Height(f64),
}

fn main() {

    // Vectors

    let _v: Vec<i32> = Vec::new();

    let mut v = vec![1,2,3]; // Use a macro to initialise

    v.push(4);
    v.push(5);

    check_fifth(&v);
    
    v.push(6);

    check_fifth(&v);

    for i in &mut v {
        *i -= 1;
        println!("Position {}", i);
    }

    let info = vec![
        FilledInfo::Age(10),
        FilledInfo::Name(String::from("Pepe")),
        FilledInfo::Height(150.12),
    ];

    for d in &info {
        println!("{:?}", d);
    }

    // Strings

    let mut s: String = String::new();
    s = "some contents".to_string();
    println!("{}", s);

    let mut s = String::from("another content");
    s.push_str(" (Appended data!)");
    println!("{}", s);

    let mut s = String::from("Lo");
    s.push('L');
    println!("{}", s);

    let s2 = String::from (" - Ciao s");
    let s3 = s + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}",s);    

    let test = "Здравствуйте";
    println!("{}", &test[0..4]); 

    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }

    //HashMaps

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let score = scores.get(&String::from("Blue"));
    match score {
        Some(si) => println!("{}", si),
        None => println!("There is no element."),
    }     
    
    println!("{:?}", scores);
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); 
    scores.entry(String::from("Blue")).or_insert(50);    

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn check_fifth(v: &Vec<i32>) {

    match v.get(5) {
        Some(si) => println!("The fifth element is {}", si),
        None => println!("There is no fifth element."),
    }

}

