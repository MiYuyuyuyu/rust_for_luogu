fn p5705() {
    let mut s=String::new();
    std::io::stdin()
        .read_line(& mut s)
        .expect("s的填充报错");
    // s.reserve(s.len());
    let reverse_s:String=s.trim().chars().rev().collect();
    println!("{}",reverse_s);
}