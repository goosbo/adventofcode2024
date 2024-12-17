use std::collections::HashMap;

struct Bot{
    pos:(isize,isize),
    velocity:(isize,isize)
}

fn main(){
    let lines = include_str!("input.txt").lines();
    let mut bots:Vec<Bot> = vec![];
    let height = 103;
    let width = 101;

    for line in lines{
        let mut b:Bot = Bot{
            pos:(0,0),
            velocity:(0,0)
        };
        
        let i = line.chars().position(|x| x== 'p').unwrap();
        let j = line.chars().position(|x| x== ',').unwrap();
        let k = line.chars().position(|x| x== ' ').unwrap();
        b.pos = (line[j+1..k].parse().unwrap(),line[i+2..j].parse().unwrap());

        let l = &line[k+1..];
        let i = l.chars().position(|x| x== 'v').unwrap();
        let j = l.chars().position(|x| x== ',').unwrap();
        b.velocity = (l[j+1..].parse().unwrap(),l[i+2..j].parse().unwrap());

        bots.push(b);
    }
    let mut iteration = 0;
    loop{
        iteration+=1;
        let mut h = HashMap::<(isize,isize),i32>::new();
        for index in 0..bots.len(){
            let i = bots[index].pos.0 + bots[index].velocity.0;
            if i >= 0 && i < height{ bots[index].pos.0 = i; }
            else{
                if i < 0 { bots[index].pos.0 = height + i; }
                else { bots[index].pos.0 = i-height; }
            }

            let j = bots[index].pos.1 + bots[index].velocity.1;

            if j >=0 && j < width{ bots[index].pos.1 = j;}
            else{
                if j < 0 { bots[index].pos.1 = width + j; }
                else { bots[index].pos.1 = j-width; }
            }

            *h.entry(bots[index].pos).or_default()+=1;
        }
        let mut flag = true;
        for (_,i) in h{
            if i > 1{
                flag = false;
                break;
            }
        }
        if flag{
            break;
        }
    }

    println!("{}",iteration);
}