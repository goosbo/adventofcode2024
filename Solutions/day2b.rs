use std::io;

fn sign(n:i32) -> i32{
    if n == 0{
        return  0;
    }
    n.abs()/n
}
fn is_valid(levels: &Vec<i32>) -> bool{
    let mut bsign = 0;
    let mut flag = true;
    for i in 0..(levels.len()-1){
        let diff = levels[i+1]-levels[i];

        if i == 0{
            bsign = sign(diff)
        }

        if diff.abs() > 3 || diff == 0 {
            flag = false;
            break;
        }
        let curr_sign = sign(diff);
        if curr_sign != bsign {
            flag = false;
            break;
        }

    }
    flag
}
fn main() {
    let mut count = 0;
    loop{
            let mut report = String::new();
            io::stdin()
                .read_line(&mut report)
                .expect("Invalid input");
            if report.trim().is_empty() {
                break;
            }
            let levels:Vec<i32>= report.trim().split_whitespace().map(|x|x.parse().expect("invalid input")).collect();

            if !is_valid(&levels){
                for i in 0..levels.len(){
                    let mut levels_clone = levels.clone();
                    levels_clone.remove(i);
                    if is_valid(&levels_clone){
                       count+=1;
                       break; 
                    }
                }
            }
            else {
                count+=1;
            }

        } 
    println!("{}",count);
}

