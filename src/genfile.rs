use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{self, BufRead, Write};
use std::path::Path;
use rand::prelude::*;

pub fn genfile(inp_file: Option<String>, 
    oup_dir: String,
    num_files: usize, 
    max_words: usize) -> io::Result<()> {
    // generate the word list
    let mut word_list: Vec<String> = vec!{
        "foo".to_string(), 
        "bar".into(), 
        "baz".into(), 
        "qux".into(), 
        "quux".into()};
    // if the input file is not empty, try to read words from the file
    if let Some(filename) = inp_file {
        let mut tmp_list: Vec<String> = vec!{};
        let lines = read_lines(filename)?;
        for line in lines {
            if let Ok(word) = line {
                tmp_list.push(word)
            }
        }
        word_list = tmp_list;
    }
    println!("available words are: {:?}", word_list);
   
    create_dir_all(&oup_dir)?;
    for i in 0..num_files {
        let mut rng = rand::thread_rng();
        let num_words = rng.gen_range(0, max_words);
        let tmp_fn = format!("{}/{}.txt", oup_dir, i);
        println!("creating file {}, which will contain {} words", 
            tmp_fn, num_words);
        let mut tmp_file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(tmp_fn)?;
        for _ in 0..num_words {
            let tmp_index = rng.gen_range(0, word_list.len());   
            writeln!(tmp_file, "{}", word_list[tmp_index])?;
        }
    }
    Ok(())
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where 
    P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
