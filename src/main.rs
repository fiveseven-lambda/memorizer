use std::io::{BufReader, BufRead, Write};
use rand::distributions::Distribution;

fn main() {
    let mut args = std::env::args();
    args.next();
    if let Some(filename) = args.next() {
        match read(filename) {
            Ok(list) => loop {
                ask(&list);
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    } else {
        println!("usage: memorizer [filename]");
    }
}

fn read(filename: String) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filename)?;
    let mut lines = BufReader::new(file).lines();

    let mut ret = Vec::new();

    loop {
        let question;
        if let Some(line) = lines.next() {
            question = line?;
        } else {
            break
        }
        let answer;
        if let Some(line) = lines.next() {
            answer = line?;
        } else {
            break
        }
        ret.push((question, answer));
    }

    Ok(ret)
}

fn ask(list : &Vec<(String, String)>) {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::new(0, list.len());
    let (question, answer) = &list[dist.sample(&mut rng)];
    print!("{}\n> ", question);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    diff(input.trim_end(), answer);
}

fn diff(s : &str, t : &str){
    if s == t {
        return
    }
    let slen = s.chars().count();
    let tlen = t.chars().count();
    let table = {
        let mut table = vec![vec![0; tlen + 1]; slen + 1];
        for (i, sc) in s.chars().enumerate() {
            for (j, tc) in t.chars().enumerate() {
                table[i + 1][j + 1] = if sc == tc {
                    table[i][j] + 1
                } else {
                    std::cmp::max(table[i][j + 1], table[i + 1][j])
                }
            }
        }
        table
    };
    let diff = {
        let mut diff = vec![true; tlen];
        let mut i = slen;
        let mut j = tlen;
        for t in (0 .. table[slen][tlen]).rev() {
            while table[i - 1][j] > t {
                i -= 1;
            }
            while table[i][j - 1] > t {
                j -= 1;
            }
            i -= 1;
            j -= 1;
            diff[j] = false;
        }
        diff
    };

    for (i, tc) in t.chars().enumerate() {
        if diff[i] {
            print!("{}", ansi_term::Style::new().bold().paint(tc.to_string()));
        } else {
            print!("{}", tc);
        }
    }
    println!("");
}
