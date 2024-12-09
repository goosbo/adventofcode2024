use std::i64;

struct FileMem {
    start_index:usize,
    length:usize,
    id:i64
}

struct FreeMem {
    start_index: usize,
    length: usize
}
fn main(){
    let line:String = String::from(include_str!("input.txt"));
    
    let line: Vec<usize> = line.trim().chars().map(|x| x.to_string().parse().unwrap()).collect();

    let mut memory: Vec<FileMem>  = vec![];
    let mut free_space: Vec<FreeMem> = vec![];
    let mut id = 0;
    let mut index = 0;
    for i in 0..line.len(){
        if i%2 == 1{
            let space: FreeMem = FreeMem{
                start_index:index,
                length: line[i]
            };
            index += space.length;
            free_space.push(space);
        }
        else{
            let mem: FileMem = FileMem{
                start_index: index,
                length: line[i],
                id:id
            };
            index+=mem.length;
            id+=1;
            memory.push(mem);
        }
        
    }

    let mut last_index = memory.len()-1;
    
    while last_index>0{
        for i in 0..free_space.len(){
            if free_space[i].length >= memory[last_index].length && memory[last_index].start_index > free_space[i].start_index{
                memory[last_index].start_index = free_space[i].start_index;
                free_space[i].start_index += memory[last_index].length;
                free_space[i].length -= memory[last_index].length;
                break;
            }
        }
        last_index-=1;
    }
    
    let mut sum = 0;

    for i in memory.iter(){
        for j in i.start_index..i.start_index+i.length{
            sum += i.id * j as i64;
        }
    }

    println!("{}",sum);

}
 