fn p5721() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let n:i32=s_temp.trim().parse().expect("err");
    let mut ans_s=String::new();
    let mut cnt=1;
    for i in 0..n{
        for _ in 0..n-i{
            let temp=format!("{:02}",cnt);
            cnt+=1;
            ans_s.push_str(&temp);
        }
        ans_s.push('\n');
    }
    println!("{}",ans_s);
}