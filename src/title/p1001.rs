fn p1001() {
    let mut s:String=String::new();
    std::io::stdin()
        .read_line(& mut s)
        .expect("s的填充报错");
    let ab:Vec<&str>=s.trim().split_whitespace().collect();
    let a:i64=ab[0].parse().expect("a的填充报错");
    let b:i64=ab[1].parse().expect("b的填充报错");
    println!("{}",a+b);
}