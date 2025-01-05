fn P1421() {
    let mut s=String::new();

    std::io::stdin()
        .read_line(& mut s)
        .expect("属于s的填充报错00");

    let ab:Vec<&str>=s.trim().split_whitespace().collect();
    let a:i32=ab[0].parse().expect("a的填充报错00");
    let b:i32=ab[1].parse().expect("b的填充报错00");

    // println!("a是{},b是{}",a,b);

    let sum=a*10+b;
    let pen_size=19;

    let ans:i32=(sum/pen_size) as i32;

    println!("{}",ans);
}