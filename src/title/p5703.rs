fn p5703() {
    let mut s:String=String::new();
    std::io::stdin()
        .read_line(& mut s)
        .expect("s的填充报错");

    let ab:Vec<&str>=s.trim().split_whitespace().collect();

    let a:i32=ab[0].parse().expect("a的填充报错");
    let b:i32=ab[1].parse().expect("a的填充报错");

    let ans=a*b;

    println!("{}",ans);
}