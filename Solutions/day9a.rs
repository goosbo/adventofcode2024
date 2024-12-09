fn main(){
    let line:String = String::from(include_str!("input.txt"));
    
    let line: Vec<i64> = line.trim().chars().map(|x| x.to_string().parse().unwrap()).collect();

    let mut v: Vec<i64> = vec![];
    let mut id = 0;
    for i in 0..line.len(){
        if i%2 == 1{
            for _j in 0..line[i]{
                v.push(-1);
            }
        }
        else{
            for _j in 0..line[i]{
                v.push(id);
            }
            id+=1;
        }
        
    }

    let mut last_index = v.len()-1;
    for i in 0..v.len(){
        if v[i] != -1{
            continue;
        }
        else {
            while v[last_index] == -1{
                last_index -=1;
            }
            if last_index < i{
                break;
            }
            v[i] = v[last_index];
            v[last_index] = -1;
        }
    }

    let sum:i64 = v.iter().enumerate().map(|(i,&x)|{
        if x == -1{
            0
        }
        else {
            x*(i as i64)
        }
    }).sum();

    println!("{}",sum);

}
 