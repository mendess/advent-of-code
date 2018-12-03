use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn get_changes(f :File) -> Vec<i32> {
    BufReader::new(f)
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect()
}

fn main() -> std::io::Result<()>{
    let mut freq :i32 = 0;
    let mut freqs = HashMap::new();
    let mut repeated = false;
    let changes = get_changes(File::open("input")?);
    while !repeated {
        for c in &changes {
            freq += c;
            if freqs.contains_key(&freq) {
                repeated = true;
                break;
            }else{
                freqs.insert(freq, 1);
            }
        }
    }

    println!("Freq: {}", freq);
    Ok(())
}
