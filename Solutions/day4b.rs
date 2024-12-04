use std::io;

fn block_count(e:&Vec<char>)-> i32{
    let mut d1 = String::new();
    let mut d2 = String::new();
    for i in 0..3{
        d1.push(e[3*i+i]);
        d2.push(e[3*i+2-i]);
    }
    let d1_rev: String = d1.chars().rev().collect();
    let d2_rev: String = d2.chars().rev().collect();
    if (d1 == "MAS" || d1_rev == "MAS") && (d2 == "MAS" || d2_rev == "MAS"){
        return 1;
    }
    0
}

fn main() {
    let mut grid : Vec<Vec<char>> = vec![];
    loop{
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("invalid input");

        if line.trim() == ""{
            break;
        }

        let l:Vec<char> = line.trim().chars().collect();
        grid.push(l);
    }
    let mut count = 0;
    for i in 0..(grid.len()-2){
        for j in 0..(grid[0].len()-2){
            let mut block: Vec<char> = vec![];
            for k in 0..3{
                for l in 0..3{
                    block.push(grid[i+k][j+l]);
                }
            }
            count+=block_count(&block);
        }
    }
    println!("{}",count);
}
