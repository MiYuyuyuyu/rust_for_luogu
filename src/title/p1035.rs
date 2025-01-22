fn p1035() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let mut cnt =2.0;
    let mut ans=1.0;
    let n:i32=s_temp.trim().parse().expect("err");
    loop {
        ans += (1.0/cnt) ;
        if ans>(n as f64){
            break;
        }
        else{
            cnt+=1.0;
        }
    }
    println!("{}",cnt);
}