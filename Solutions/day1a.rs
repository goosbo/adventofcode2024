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
    list1.sort();
    list2.sort();
    let mut sum = 0;
    for (n1,n2) in list1.iter().zip(list2.iter()){
        let diff = n1-n2;
        sum += diff.abs();
    }
    println!("{}",sum);

}
