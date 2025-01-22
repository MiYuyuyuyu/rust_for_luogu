fn p1009() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let n:i32=s_temp.trim().parse().expect("err");
    let mut ans:i32=0;
    for i in 1..n+1{
        let mut temp=1;
        for j in 1..i+1{
            temp*=j;
        }
        ans+=temp;
    }
    println!("{}",ans);
}