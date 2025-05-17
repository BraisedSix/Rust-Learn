use std::env;
use std::fs;
use std::io;

fn count_stats(content: &str) -> (usize, usize, usize) {
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    (lines, words, chars)
}

pub fn run_w() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [-w] <file>", args[0]);
        return Ok(());
    }

    let (flag, filename) = if args[1] == "-w" {
        if args.len() < 3 {
            eprintln!("Usage: {} -w <file>", args[0]);
            return Ok(());
        }
        (true, &args[2])
    } else {
        (false, &args[1])
    };

    let content = fs::read_to_string(filename)?;
    let (lines, words, chars) = count_stats(&content);
    if flag {
        println!("{}", words);
    } else {
        println!("{} {} {} {}", lines, words, chars, filename);
    }
    Ok(())
}
