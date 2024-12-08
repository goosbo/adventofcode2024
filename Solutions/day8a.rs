use std::io;
use std::collections::HashMap;

fn main(){
    let mut antennas:HashMap<char, Vec<(usize,usize)>>= HashMap::new();
    let mut grid:Vec<Vec<char>> = vec![];
    loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim() == ""{
            break;
        }

        grid.push(line.trim().chars().collect());
    }


    for i in 0..grid.len(){
        for j in 0..grid[i].len(){
            if grid[i][j] != '.'{
                match antennas.get_mut(&grid[i][j]) {
                    Some(freq) =>{
                        freq.push((i,j));
                    },
                    None =>{
                        antennas.insert(grid[i][j], vec![(i,j)]);
                    }
                }
            }
        }
    }

    let mut antinodes = grid.clone();

    for (_freq, fantennas) in antennas.iter(){
        for i in 0..fantennas.len(){
            for j in i+1..fantennas.len(){
                let coord0:(isize,isize) = (fantennas[i].0 as isize,fantennas[i].1 as isize);
                let coord1:(isize,isize) = (fantennas[j].0 as isize,fantennas[j].1 as isize);

                let delta_i:isize = coord1.0 - coord0.0;
                let delta_j:isize = coord1.1 - coord0.1;

                let antinode0 = (coord0.0-delta_i,coord0.1-delta_j);
                if (antinode0.0 >= 0 && antinode0.0 < grid.len() as isize) && (antinode0.1 >= 0 && antinode0.1 < grid[0].len() as isize){
                    antinodes[antinode0.0 as usize][antinode0.1 as usize] = '#';
                }

                let antinode1 = (coord1.0+delta_i,coord1.1+delta_j);
                if (antinode1.0 >= 0 && antinode1.0 < grid.len() as isize) && (antinode1.1 >= 0 && antinode1.1 < grid[0].len() as isize){
                    antinodes[antinode1.0 as usize][antinode1.1 as usize] = '#';
                }
            }
        }
    }

    let mut count = 0;

    for i in 0..antinodes.len(){
        for j in 0..antinodes[i].len(){
            if antinodes[i][j] == '#'{
                count+=1;
            }
        }
    }
    println!("{}",count);
}