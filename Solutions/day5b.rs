use std::io;

fn is_correct_order(v: &Vec<i32>,rules: &Vec<(i32,i32)>)-> bool{
    let mut flag = true;
    for i in 0..v.len()-1{
        if rules.contains(&(v[i+1],v[i])){
            flag = false;
            break;
        }
    }
    flag
}

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
    let mut incorrect_updates: Vec<Vec<i32>> = vec![];
    loop{
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("invalid input");
        if line.trim() == ""{
            break;
        }

        let v: Vec<i32> = line.trim().split(",").map(|x| x.parse().unwrap()).collect();
        if !is_correct_order(&v, &rules){
            incorrect_updates.push(v);
        }

    }
    let mut sum = 0;

    for i in 0..incorrect_updates.len(){
        while !is_correct_order(&incorrect_updates[i], &rules){
            for j in 0..incorrect_updates[i].len()-1{
                if rules.contains(&(incorrect_updates[i][j+1],incorrect_updates[i][j])){
                    incorrect_updates[i].swap(j, j+1);
                }
            }
        }
        sum += incorrect_updates[i][incorrect_updates[i].len()/2];
    }

    println!("{}",sum);
}
