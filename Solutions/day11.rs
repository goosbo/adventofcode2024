use std::collections::HashMap;
fn main(){
    let line= include_str!("input.txt").trim().split_whitespace();
    let mut stones: HashMap<i64,i64> = HashMap::new();
    for l in line{
        let t: i64 = l.parse().unwrap();
        match stones.get_mut(&t){
            Some(n)=>{
                *n +=1;
            }
            None=>{
                stones.insert(t, 1);
            }
        }
    }
    
    let iterations = 75;
    
    for i in 0..iterations{
        if i == 25{
            println!("Part A: {}",stones.values().sum::<i64>());
        }

        let mut stones_update:HashMap<i64, i64> = HashMap::with_capacity(stones.len());
        for (number,count) in stones.into_iter(){
            if number == 0{
               *stones_update.entry(1).or_default() += count;
            }
            else if number.to_string().len()%2==0{
                let length = number.to_string().len() as u32;
                *stones_update.entry(number%(10i64.pow(length/2))).or_default() += count;
                *stones_update.entry(number/(10i64.pow(length/2))).or_default() += count;
            }
            else{
                *stones_update.entry(number*2024).or_default() +=count;
            }
        }
        stones = stones_update;
    }

    println!("Part B: {}",stones.values().sum::<i64>());
}
 