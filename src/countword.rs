use std::thread;
use std::fs;
use std::io;
use std::collections;
use super::genfile;
use std::sync::{Arc, Mutex};

pub fn count_word_mutex(inp_dir: String) -> io::Result<()>{
    // 1. create a global map
    let word_count = collections::HashMap::<String, u8>::new();
    let wcm = Arc::new(Mutex::new(word_count));
    // 2. list files in the inp_dir
    let paths = fs::read_dir(inp_dir)?;
    let mut handles = vec!{};
    for result_path in paths {
        let path = result_path?;
        let tmp_wcm = wcm.clone();
        // 3. for each file, spawn a new thread
        handles.push(thread::spawn(|| count_word(path, tmp_wcm)));
    }

    for handle in handles {
        handle.join().unwrap()
    }
    println!("{:?}", wcm.lock().unwrap());
    Ok(())
}

fn count_word(path: fs::DirEntry, wcm: Arc<Mutex<collections::HashMap<String, u8>>>) {
    let lines = genfile::read_lines(path.path()).unwrap();
    for line in lines {
        let mut tmp_wc = wcm.lock().unwrap();
        if let Ok(word) = line {
            match tmp_wc.get_mut(&word) {
                Some(c) => {*c += 1;},
                None => {tmp_wc.insert(word, 1);},
            };
        }
    }
}
