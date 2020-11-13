use std::thread;
use std::fs;
use std::io;
use std::collections;
use super::genfile;
use std::sync::{Arc, Mutex, mpsc};

pub fn count_word_mutex(inp_dir: String) -> io::Result<()>{
    // 1. create a global map
    let word_count = collections::HashMap::<String, u32>::new();
    let wcm = Arc::new(Mutex::new(word_count));
    // 2. list files in the inp_dir
    let paths = fs::read_dir(inp_dir)?;
    let mut handles = vec!{};
    for result_path in paths {
        let path = result_path?;
        let tmp_wcm = wcm.clone();
        // 3. for each file, spawn a new thread
        handles.push(thread::spawn(|| count_word_lock(path, tmp_wcm)));
    }

    for handle in handles {
        handle.join().unwrap()
    }
    println!("{:?}", wcm.lock().unwrap());
    Ok(())
}

pub fn count_word_mpsc(inp_dir: String) -> io::Result<()> {
    // 1. create a global map
    let mut word_count = collections::HashMap::<String, u32>::new();
    // 2. create the channel
    let (tx, rx): (mpsc::Sender<(String, u32)>, mpsc::Receiver<(String, u32)>) = mpsc::channel();
    let paths = fs::read_dir(inp_dir)?;
    for result_path in paths {
        // 3. for every file, spawn a thread with a transmitter 
        let path = result_path?;
        let tmp_tx = tx.clone();
        thread::spawn(|| count_word_channel(path, tmp_tx));
    }
    drop(tx);
    // 4. read from the receiver
    for received in rx {
        let(word, count) = received;
        match word_count.get_mut(&word) {
            Some(c) => { *c += count; },
            None => {word_count.insert(word, count);},
        }
    }
    println!("{:?}", word_count);
    Ok(())
}

fn count_word_channel(path: fs::DirEntry, tx: mpsc::Sender<(String, u32)>) {
    let lines = genfile::read_lines(path.path()).unwrap();
    let mut word_count = collections::HashMap::<String, u32>::new();
    for line in lines {
        if let Ok(word) = line {
            match word_count.get_mut(&word) {
                Some(c) => {*c+=1;},
                None => {word_count.insert(word, 1);},
            }
        }
    }
    for (key, value) in word_count {
        tx.send((key, value)).unwrap();
    }
}

fn count_word_lock(path: fs::DirEntry, wcm: Arc<Mutex<collections::HashMap<String, u32>>>) {
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
