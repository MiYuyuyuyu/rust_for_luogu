fn p3954() {
    //a,b,c
    //20 30 50
    let mut s=String::new();
    std::io::stdin()
        .read_line(& mut s)
        .expect("s的填充报错");
    let abc:Vec<&str>=s.trim().split_whitespace().collect();

    let a:i32=abc[0].parse().expect("a的填充报错");
    let b:i32=abc[1].parse().expect("b的填充报错");
    let c:i32=abc[2].parse().expect("c的填充报错");

    let ans=( (a as f32)*0.2 +(b as f32)*0.3 + (c as f32)*0.5 ) as i32;
    println!("{}",ans);
}