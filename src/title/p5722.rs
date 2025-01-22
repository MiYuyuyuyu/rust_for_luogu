fn p5722() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let n:i32=s_temp.trim().parse().expect("err");
    let mut ans=0;
    for i in 0..n+1{
        ans+=i;
    }
    println!("{}",ans);
}