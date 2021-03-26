//use std::env;
use std::fs::File;
use std::vec;
use std::io::{BufReader, BufRead, Error, Write};

struct Question {
    id: u16,
    qtype: usize,
    text: String,
    ans: Vec<String>,
}

fn read_db() {

}

fn print_question(q: &Question) {
    println!("{}", q.text);

    for(pos, answer) in q.ans.iter().enumerate(){
        println!("{}: {}", (pos + 97).to_string(), answer);
    }
}

fn parse_test_args(argv: Vec<&str>) -> Vec<String> {    //credit ntrepid8
    argv.iter().map(|&s| s.to_string()).collect::<Vec<String>>()
}

fn load_questions() -> Result<Vec<Question>, Error> {
    let path = "assets/db.txt";

    let fileinstance = File::open(path)?;
    let mut bufferedfile = BufReader::new(fileinstance);
    
    let mut questions = Vec::new();
    let mut header = String::new();
    let mut dbvol = String::new();

    bufferedfile.read_line(&mut header).expect("Unable to read header");

    println!("Header: {}", header);

    bufferedfile.read_line(&mut dbvol).expect("Unable to read db value");

    println!("DB value: {}", dbvol);

    for line in bufferedfile.lines() {
        let txt: String = line.expect("Unable to read question");
        let qsplit: Vec<&str> = txt.split('分').collect();
        let anssplit: Vec<String> = parse_test_args(qsplit[1].split("円").collect());


        
        questions.push(Question{id: 1, qtype: anssplit.len(), text:qsplit[0].to_string(), ans:anssplit});
    }

    Ok(questions)
}

fn get_answer(q: &Question) -> bool {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let value: String = input.trim().to_string();

    //println!("{}", value);
    
    if value == q.ans[0] {
        true
    }
    else {
        false
    }
}

fn main() -> Result<(), Error> {
    println!("Hello");

    let questions = load_questions().expect("Error loading questions");

    let q1 = Question {
        id: 1,
        qtype: 2,
        text: "What is A?".to_string(),
        ans: vec!["A".to_string(), "B".to_string()]
    };

    
    print_question(&q1);

    if get_answer( &q1 ) {
        println!("Good job!");
    }
    else {
        println!("Idiot");
    }

    for (pos, question) in questions.iter().enumerate(){
        print_question(&question);

        if get_answer( &question ) {
            println!("Good job!");
        }
        else {
            println!("Idiot");
        }
    }

    Ok(())

    // --snip--
}