fn main() {
    let mut s_temp = String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .unwrap();
    let s1 = s_temp.trim().to_string();
    s_temp.clear();
    std::io::stdin()
        .read_line(&mut s_temp)
        .unwrap();
    let s2 = s_temp.trim().to_string();
    let mut ans1 = 1;
    let mut ans2 = 1;
    for i in s1.chars(){
        ans1*=(i as u8 -b'A'+1) as u32;
        ans1%=47;
    }
    for i in s2.chars(){
        ans2*=(i as u8 -b'A'+1) as u32;
        ans2%=47;
    }
    if ans1%47 == ans2%47{
        println!("GO");
    }
    else{
        println!("STAY");
    }
}