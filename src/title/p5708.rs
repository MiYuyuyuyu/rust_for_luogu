fn p5708() {
    let mut s:String=String::new();
    std::io::stdin()
        .read_line(& mut s)
        .expect("s的填充报错");

    let abc:Vec<&str>=s.trim().split_whitespace().collect();

    let a:f64=abc[0].parse().expect("a的填充报错");
    let b:f64=abc[1].parse().expect("a的填充报错");
    let c:f64=abc[2].parse().expect("c的填充报错");

    let p = (a + b + c) / 2.0;

    let mut ans = p * (p - a) * (p - b) * (p - c);
    
    ans=ans.sqrt();

    println!("{:.1}",ans);
}