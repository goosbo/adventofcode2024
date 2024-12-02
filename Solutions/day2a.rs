use std::io;

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
        let mut sign = true;
        let mut flag = true;
        for i in 0..(levels.len()-1){
            let diff = levels[i+1]-levels[i];

            if i == 0{
                sign = diff > 0;
            }

            if diff.abs() > 3 || diff == 0 {
                flag = false;
                break;
            }
            let curr_sign = diff>0;
            if curr_sign != sign {
                flag = false;
                break;
            }

        }
        if flag{
            count+=1;
        }
    }
    println!("{}",count);

}
