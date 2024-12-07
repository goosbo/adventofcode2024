use std::io;

fn is_possible(test_value:i64,v:&Vec<i64>,index:usize,val:i64)->bool{
    if index == v.len(){
        return test_value == val;
    }
    if val > test_value{
        return  false;
    }
    let base:i64 = 10;
    let power= v[index].checked_ilog10().unwrap_or(0)+1;
    return is_possible(test_value, v, index+1, val+v[index]) || is_possible(test_value, v, index+1, val*v[index]) || is_possible(test_value, v, index+1, val*base.pow(power)+v[index]);
}

fn main(){
    let mut sum = 0;
    
    loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == ""{
            break;
        }
        let nums:Vec<&str> = line.trim().split(":").collect();
        let test_value:i64 = nums[0].parse().unwrap();
        let v: Vec<i64> = nums[1].split_whitespace().map(|x| x.parse().unwrap()).collect();
        if is_possible(test_value, &v, 1,v[0] ){
            sum+=test_value;
        }
    }
    println!("{}",sum);
}