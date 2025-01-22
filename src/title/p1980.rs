fn p1980() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let s:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let n:i32=s[0].trim().parse().expect("err");
    let x:i32=s[1].parse().expect("err");
    let mut cnt =0;
    let mut s=String::new();
    for i in 1..n+1{
        let temp=format!("{}",i);
        // println!("{}",i);
        s.push_str(& temp);
    }
    // println!("{}",s);
    // println!("{}",((b'0' as u8) + (x as u8))as char);
    let c:Vec<char>=s.chars().collect();
    for i in c {
        if i as u8 == (b'0' as u8) + (x as u8) {
            cnt+=1;
        }
    }
    println!("{}",cnt);
}