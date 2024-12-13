fn main(){
    let lines = include_str!("input.txt").lines();

    let mut a:Vec<(f64,f64)> = vec![];
    let mut b:Vec<(f64,f64)> = vec![];
    let mut prizes:Vec<(f64,f64)> = vec![];

    for line in lines{
        if line.contains("A"){
           let i = line.chars().position(|x| x == 'X').unwrap();
           let j = line.chars().position(|x|x==',').unwrap();
           let e:i64 = line[i+2..j].parse().unwrap();

           let i = line.chars().position(|x| x=='Y').unwrap();
           let f:i64 = line[i+2..].trim().parse().unwrap();
           a.push((e as f64,f as f64));
        }
        else if line.contains("B"){
            let i = line.chars().position(|x| x == 'X').unwrap();
           let j = line.chars().position(|x|x==',').unwrap();
           let e:i64 = line[i+2..j].parse().unwrap();

           let i = line.chars().position(|x| x=='Y').unwrap();
           let f:i64 = line[i+2..].trim().parse().unwrap();
           b.push((e as f64,f as f64));
        }
        else if line.contains("Prize"){
            let i = line.chars().position(|x| x == 'X').unwrap();
           let j = line.chars().position(|x|x==',').unwrap();
           let e:i64 = line[i+2..j].parse().unwrap();

           let i = line.chars().position(|x| x=='Y').unwrap();
           let f:i64 = line[i+2..].trim().parse().unwrap();
           prizes.push((e as f64,f as f64));
        }
    }
    let mut tokens = 0;
    for i in 0..a.len(){
        prizes[i].0+= 10000000000000.0;
        prizes[i].1 += 10000000000000.0;
        let det = a[i].0*b[i].1-a[i].1*b[i].0;
        let x = (b[i].1*prizes[i].0 - b[i].0*prizes[i].1)/det;
        let y = (a[i].0*prizes[i].1 - a[i].1*prizes[i].0)/det;
        if x.fract() != 0.0 || y.fract()!= 0.0{continue;}
        tokens+= (x as i64)*3+(y as i64);
    }
    println!("{}",tokens);
}
 