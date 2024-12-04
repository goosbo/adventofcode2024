use std::io;

fn row_count(e:&Vec<char>)-> i32{
    let mut c = 0;
    for j in 0..(e.len()-3){
        let h:String = e[j..j+4].iter().collect();
        let h_rev:String = h.chars().rev().collect();
        if h == "XMAS" || h_rev == "XMAS"{
            c+=1;
        }
    }
    c
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
    for i in 0..(grid.len()){
        count+= row_count(&grid[i]);
    }
    for j in 0..(grid[0].len()){
        let mut vert: Vec<char> = vec![];
        for i in 0..(grid.len()){
            vert.push(grid[i][j]);
        }
        count+= row_count(&vert);
    }
    for n in 3..2*grid.len()-4{
        let mut d1: Vec<char> = vec![];
        for i in 0..(n+1){
            if i >= grid.len() || n-i >= grid.len(){
                continue;
            }
            d1.push(grid[i][n-i]);
        }
        count+= row_count(&d1);
    }
    for n in 0..grid.len()-3{
        let mut d2: Vec<char> = vec![];
        for i in 0..(grid.len()-n+1){
            if i >= grid.len() || i+n >= grid.len(){
                continue;
            }
            d2.push(grid[i][i+n]);
        }
        count+=row_count(&d2);
    }
    for n in 1..grid.len()-3{
        let mut d2: Vec<char> = vec![];
        for i in n..grid.len(){
            if i >= grid.len() || i-n >= grid.len(){
                continue;
            }
            d2.push(grid[i][i-n]);
        }
        count+=row_count(&d2);
    }

    println!("{}",count);
}
