use std::io;
fn main() {
    let mut list1:Vec<i64> = Vec::new();
    let mut list2:Vec<i64> = Vec::new();

    loop{
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Invalid input");
        if line.trim().is_empty() {
            break;
        }
        let numline:Vec<&str> = line.trim().split_whitespace().collect();
        list1.push(numline[0].parse().expect("Invalid input"));
        list2.push(numline[1].parse().expect("Invalid input"));
    }
    let mut sum = 0;
    for n1 in list1.iter(){
        let mut count = 0;
        for n2 in list2.iter(){
            if n1 == n2{
                count+=1;
            }
        }
        sum += n1 *count;
    }
    println!("{}",sum);

}
