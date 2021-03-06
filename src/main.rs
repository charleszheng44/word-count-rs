use clap::Clap;
mod genfile;
mod countword;
use std::time::Instant;

#[derive(Clap)]
#[clap(version = "1.0", author = "Chao Zheng <charleszheng44@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.0", author = "Chao Zheng <charleszheng44@gmail.com>")]
    GenFile(FileGenerator),
    CountWord(WordCounter),
}

/// A subcommand for generating files that contains random words
#[derive(Clap)]
struct FileGenerator {
    #[clap(short, long)]
    words_file: Option<String>,
    #[clap(short, long, default_value = "1")]
    num_files: usize,
    #[clap(short, long, default_value = "1")]
    max_num_words: usize,
    #[clap(short, long, default_value = "word_count_tmp")]
    oup_dir: String
}

/// A subcommand for counting words in the files under the directory
#[derive(Clap)]
struct WordCounter {
    #[clap(short, long)]
    inp_dir: String,
    #[clap(short, long, default_value = "mutex")]
    mode: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    
    match opts.subcmd {
        SubCommand::GenFile(fg) => { 
            match fg.words_file.as_ref() {
                None => {
                    print!("will use words in the default word list ")
                },
                Some(filename) => {
                    print!("will use words in file {} ", filename);
                }
            };
            println!("to create {} files and store them in {}.", 
                fg.num_files, fg.oup_dir);
            println!("Each file will contain at most {} words.", fg.max_num_words);
            genfile::genfile(fg.words_file, 
                fg.oup_dir, 
                fg.num_files, 
                fg.max_num_words).unwrap();
        }
        SubCommand::CountWord(wc) => {
            let before =Instant::now();
            match wc.mode.as_str() {
                "mutex" => {countword::count_word_mutex(wc.inp_dir).unwrap();},
                "channel" => {countword::count_word_mpsc(wc.inp_dir).unwrap();},
                "seq" => {countword::count_word_seq(wc.inp_dir).unwrap();},
                _ => {panic!("unknown mode {}, available modes are: mutex, channel, seq", wc.mode.as_str());},
            }
            println!("Count words took: {:.2?}", before.elapsed());
        }
    }
}
