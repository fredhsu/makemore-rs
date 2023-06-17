use std::{
    collections::{BTreeMap, HashMap},
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
    let mut b = HashMap::new();
    let mut chs2 = vec![];
    for word in lines.iter() {
        // need to treat each character of word as a string, then concat that with starting and
        // ending string
        let w: Vec<String> = word.chars().map(|x| x.to_string()).collect();
        let chs = vec![vec!["<S>".to_string()], w, vec!["<E>".to_string()]]
            .into_iter()
            .flatten()
            .collect::<Vec<String>>();
        chs2.resize(chs.len() - 1, "".to_string());
        chs2.clone_from_slice(&chs[1..]);

        let bigrams = chs.clone().into_iter().zip(chs2.clone().into_iter());
        //for bigram in chs.clone().into_iter().zip(chs2[1..].into_iter()) {
        for bigram in bigrams {
            //dbg!(&bigram);
            b.entry(bigram).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    assert_eq!(b[&("<S>".to_string(), "e".to_string())], 1531);
    let mut count_vec: Vec<_> = b.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("{:?}", count_vec);
}
