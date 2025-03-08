fn main() {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp).unwrap();
    let c: Vec<&str> = s_temp.trim().split_whitespace().collect();
    let c: Vec<i128> = c.iter().map(|i| i.parse::<i128>().unwrap()).collect();
    let n = c.len();
    let mut sum =0;
    for i in c {
        sum+=i;
    }
    sum*=2_i128.pow((n-1) as u32 );
    println!("{}", sum);
}