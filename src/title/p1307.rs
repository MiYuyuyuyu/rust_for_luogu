fn p1307() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let mut n:i32=s_temp.trim().parse().expect("err");
    if n!=0 {
        loop{
            let temp=n%10;
            if temp!=0 {
                break;
            }
            n/=10;
        }
    }
    else{
        println!("0");
        return;
    }
    
    let mut ans_s:String=String::new();
    if n<0{
        ans_s.push('-');
    }
    n=n.abs();
    s_temp=n.to_string();
    s_temp=s_temp.chars().rev().collect();
    ans_s.push_str(& s_temp);

    println!("{}",ans_s);
}