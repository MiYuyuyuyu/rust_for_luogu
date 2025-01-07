fn p5706() {
    let mut s:String=String::new();
    std::io::stdin()
        .read_line(& mut s)
        .expect("s的填充报错");

    let ab:Vec<&str>=s.trim().split_whitespace().collect();

    let a:f64=ab[0].parse().expect("a的填充报错");
    let b:i32=ab[1].parse().expect("a的填充报错");

    let ans1=a/(b as f64);
    let ans2=b*2;

    println!("{:.3}\n{}",ans1,ans2);//使用:.x来保留几位小数
}