fn trailhead_score(grid: &mut Vec<Vec<i32>>, coord: &(isize,isize))->u32{
    let i = coord.0 as usize;
    let j = coord.1 as usize;
    if grid[i][j] == 9{
        grid[i][j] = -1;
        return 1;
        
    }
    let dir: Vec<(isize,isize)> = vec![(-1,0),(0,1),(1,0),(0,-1)];
    let mut score = 0;
    for k in 0..4{
        let e = (coord.0+dir[k].0,coord.1+dir[k].1);
        if e.0 < 0 || e.0 >= grid.len() as isize || e.1 < 0 || e.1 >= grid[0].len() as isize{
            continue;
        }
        if grid[e.0 as usize][e.1 as usize] != grid[i][j]+1{
            continue;
        }
        score += trailhead_score(grid, &e);
    }
    return score;
}


fn main(){
    let lines= include_str!("input.txt").split("\n");
    let mut grid : Vec<Vec<i32>> = vec![];

    for i in lines.into_iter(){
        let mut v: Vec<i32> = vec![];
        for j in i.chars(){
            v.push(j.to_digit(10).unwrap() as i32);
        }
        grid.push(v);
    }

    let mut total_score = 0;

    for i in 0..grid.len(){
        for j in 0..grid[0].len(){
            if grid[i][j] == 0{
                let mut temp = grid.clone();
                total_score+= trailhead_score(&mut temp, &(i as isize,j as isize));
            }
        }
    }
    println!("{}",total_score);
}
 