use std::io;

fn change_direction(dir:&mut (isize,isize)){
    if dir.1 == 1 || dir.1 == -1{
        dir.0 = dir.1;
        dir.1 = 0;
    }
    else {
        dir.1 = -dir.0;
        dir.0 = 0;
    }
}

fn initial_guard_coord(grid: &Vec<Vec<char>>)->(usize,usize){
    for i in 0..grid.len(){
        for j in 0..grid[i].len(){
            if grid[i][j] == '^'{
                return (i,j);
            }
        }
    }
    return (0,0);
}

fn is_outofbounds(e:(isize,isize),grid: &Vec<Vec<char>>)-> bool{
    if e.0 >= grid.len() as isize|| e.0 < 0{
        return true;
    }
    if e.1 >= grid[0].len() as isize|| e.1 < 0{
        return true;
    }
    return false;
}

fn check_loop(coord: &(usize,usize),grid: &Vec<Vec<char>>)-> bool{
    let mut i = coord.0;
    let mut j = coord.1;
    let mut dir: (isize,isize) = (-1,0);
    let max_count = grid.len()*grid[0].len();
    let mut c = 0;
    loop{
        if is_outofbounds((i as isize+dir.0,j as isize+dir.1), &grid){
            break;
        }
        if c > max_count{
            return true;
        }
        else if grid[(i as isize+dir.0) as usize][(j as isize+dir.1) as usize] == '#'{
            change_direction(&mut dir);
            continue;
        }
        else {
            let newi = i as isize;
            let newj = j as isize;
            i = (newi + dir.0) as usize;
            j = (newj + dir.1) as usize;
            c+=1;

        }
    }
    false
}

fn main() {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut dir: (isize,isize) = (-1,0);
    loop {
        let mut l = String::new();
        io::stdin()
            .read_line(&mut l)
            .expect("invalid input");
        if l.trim() == ""{
            break;
        }
        grid.push(l.trim().chars().collect());
    }
    let mut marked_grid = grid.clone();
    let coord = initial_guard_coord(&grid);
    let mut i = coord.0;
    let mut j = coord.1;
    loop{
        if is_outofbounds((i as isize+dir.0,j as isize+dir.1), &grid){
            break;
        }
        else if grid[(i as isize+dir.0) as usize][(j as isize+dir.1) as usize] == '#'{
            change_direction(&mut dir);
            continue;
        }
        else {
            let newi = i as isize;
            let newj = j as isize;
            i = (newi + dir.0) as usize;
            j = (newj + dir.1) as usize;
            let mut new_grid = grid.clone();
            new_grid[i][j] = '#';
            if check_loop(&coord, &new_grid) {
                marked_grid[i][j] = 'x';
            }
    
        }
    }
    let mut count = 0;
    for i in 0..marked_grid.len(){
        for j in 0..marked_grid[i].len(){
            if marked_grid[i][j] == 'x'{
                count+=1;
            }
        }
    }

    println!("{}",count);
}
