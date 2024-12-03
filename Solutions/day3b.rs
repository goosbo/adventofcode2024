use std::io;
use regex::Regex;

fn main() {
    let mut program = String::new();
    loop{
        let mut l = String::new();
        io::stdin()
            .read_line(&mut l)
            .expect("invalid input");
        if l.trim() == "" {
            break;
        }
        program.push_str(&l);
    }
    let mut sum = 0;
    let mul_re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let matches: Vec<_> = mul_re.find_iter(&program).map(|x|x.as_str()).collect();
    let mut skip = false;
    for m in matches {
        if m == "do()" {
            skip = false;
            continue;
        }
        if m == "don't()"{
            skip = true;
            continue;
        }
        if skip {
            continue;
        }
        let comma = m.chars().position(|c| c == ',').unwrap();
        let n1:i64= m[4..comma].parse().unwrap();
        let n2:i64 = m[comma+1..m.len()-1].parse().unwrap();
        sum += n1*n2;
    }
    println!("{}",sum);
}
