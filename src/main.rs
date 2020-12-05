fn main() {
    let v = vec![10,20,30];

    mean(v);
}

fn mean(v:Vec<i32>) {
    let mut count = 0;
    let mut sum=0;

    for i in &v {
        count += 1;
        sum += i;
        //println!("{}: {}", count, i);
    }
    println!("The mean is {}.",sum/count)
}