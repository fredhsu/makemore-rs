use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let filename = "names.txt";
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    for i in 0..10 {
        println!("{}", &lines[i]);
    }
    println!("{} words", lines.len());
    let min = lines.iter().min_by(|x, y| x.len().cmp(&y.len())).unwrap();
    println!("min length word: {} ", min.len());
    let max = lines.iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap();
    println!("max length word: {} ", max.len());

    // starting with bigram language model
    // go through each word and pull out bigram
    for word in lines[0..3].iter() {
        // need to treat each character of word as a string, then concat that with starting and
        // ending string
        let w = word.chars().map(|x| x.to_string()).collect();
        let chs = vec![vec!["<S>".to_string()], w, vec!["<E>".to_string()]]
            .into_iter()
            .flatten()
            .collect::<Vec<String>>();

        //for (ch1, ch2) in word.chars().zip(word[1..].chars()) {
        for (ch1, ch2) in chs.clone().into_iter().zip(chs[1..].into_iter()) {
            println!("{ch1}, {ch2}");
        }
    }
}
