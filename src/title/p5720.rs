fn p5720() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let mut a:i32=s_temp.trim().parse().expect("err");
    let mut cnt=1;
    if a!=1{
        loop{
            a/=2;
            cnt+=1;
            if a==1 {
                break;
            }
        }
    }
    println!("{}",cnt);
}