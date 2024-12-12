fn floodfill(grid: &Vec<Vec<char>>,new_grid: &mut Vec<Vec<i32>>,i:usize,j:usize,region:i32){
    new_grid[i][j] = region;
    let dir:Vec<(isize,isize)> = vec![(-1,0),(0,1),(1,0),(0,-1)];
    for k in 0..4{
        let ni = i as isize + dir[k].0;
        let nj = j as isize + dir[k].1;
        if ni >=0 && nj >= 0 && ni < new_grid.len() as isize && nj < new_grid[0].len() as isize && grid[ni as usize][nj as usize] == grid[i][j] && new_grid[ni as usize][nj as usize] != region{
            floodfill(grid, new_grid, ni as usize, nj as usize, region);
        }
    }
}

fn main(){
    let lines = include_str!("input.txt").lines();
    let mut grid:Vec<Vec<char>> = vec![];

    for line in lines{
        grid.push(line.trim().chars().collect());
    }

    let mut igrid: Vec<Vec<i32>> = vec![];

    for _ in grid.iter(){
        let mut v:Vec<i32> = vec![];
        for _ in grid[0].iter(){
            v.push(0);
        }
        igrid.push(v);
    }

    let mut region_count = 1;
    for i in 0..grid.len(){
        for j in 0..grid[i].len(){
            if igrid[i][j]==0{
                let mut temp = igrid.clone();
                floodfill(&grid, &mut temp, i, j, region_count);
                igrid = temp.clone();
                region_count+=1;
            }
        }
    }

    let mut fence_dimensions:Vec<(i32,i32)> = vec![];

    for _ in 0..(region_count-1){
        fence_dimensions.push((0,0));
    }

    for i in 0..igrid.len(){
        for j in 0..igrid[i].len(){
            let mut border_vertices:Vec<(isize,isize)> = vec![];
            fence_dimensions[igrid[i][j] as usize-1].1 +=1; 

            let dir:Vec<(isize,isize)> = vec![(-1,0),(0,1),(1,0),(0,-1)];
            for k in 0..4{
                let ni = i as isize + dir[k].0;
                let nj = j as isize + dir[k].1;
                if ni < 0 || ni >= igrid.len() as isize|| nj < 0 || nj >= igrid[0].len() as isize ||igrid[ni as usize][nj as usize] != igrid[i][j]{
                    border_vertices.push((ni,nj));
                }
            }

            if border_vertices.len() ==4 {fence_dimensions[igrid[i][j] as usize-1].0+=4;}

            else if border_vertices.len() == 3 {fence_dimensions[igrid[i][j] as usize-1].0+=2; }

            else if border_vertices.len() == 2 {
                if border_vertices[1].0.abs_diff(border_vertices[0].0) != 2 && border_vertices[1].1.abs_diff(border_vertices[0].1) != 2{
                    fence_dimensions[igrid[i][j] as usize-1].0+=1;
                }
            }
            
            let diag_dir:Vec<(isize,isize)> = vec![(-1,-1),(1,1),(1,-1),(-1,1)];
            for k in 0..4{
                let ni = i as isize + diag_dir[k].0;
                let nj = j as isize + diag_dir[k].1;
                if ni >=0 && ni <igrid.len() as isize&& nj >=0 && nj < igrid[0].len() as isize && igrid[ni as usize][nj as usize] != igrid[i][j] && igrid[ni as usize][j] == igrid[i][j] && igrid[i][nj as usize] == igrid[i][j]{
                    fence_dimensions[igrid[i][j] as usize-1].0+=1;
                }
            }
        }
    }

    let price:i32 = fence_dimensions.into_iter().map(|(p,a)|p*a).sum();
    println!("{}",price);

}
 