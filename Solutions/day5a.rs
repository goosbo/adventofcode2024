use std::io;

fn main() {
    let mut rules : Vec<(i32,i32)> = vec![];
    loop{
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("invalid input");

        if line.trim() == ""{
            break;
        }
        let v: Vec<&str> = line.trim().split("|").collect();
        let a: i32 = v[0].parse().unwrap();
        let b: i32 = v[1].parse().unwrap();
        rules.push((a,b));
    }
    let mut sum:i32 = 0;
    loop{
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("invalid input");
        if line.trim() == ""{
            break;
        }

        let v: Vec<i32> = line.trim().split(",").map(|x| x.parse().unwrap()).collect();
        let mid:i32 = v[v.len()/2];
        let mut flag = true;
        for i in 0..v.len()-1{
            if rules.contains(&(v[i+1],v[i])){
                flag = false;
                break;
            }
        }
        if flag{
            sum += mid;
        }
    }
    println!("{}",sum);
}
