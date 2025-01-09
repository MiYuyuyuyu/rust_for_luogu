fn p1425() {
    // a b c d
    let mut s:String=String::new();
    std::io::stdin()
        .read_line(& mut s)
        .expect("s的填充报错");
    let c:Vec<&str>=s.trim().split_whitespace().collect();
    let a:i32=c[0].parse().expect("in a err");
    let b:i32=c[1].parse().expect("in b err");
    let d:i32=c[3].parse().expect("in d err");
    let c:i32=c[2].parse().expect("in c err");

    let sum_1=a*60+b;
    let sum_2=c*60+d;

    let dis=sum_2-sum_1;

    let ans_h=dis/60;
    let ans_m=dis%60;
    println!("{} {}",ans_h,ans_m);
}