fn p5704() {
    let mut s:String=String::new();
    std::io::stdin()
        .read_line(& mut s)
        .expect("s的填充报错");

    let c:Vec<char> = s.trim().chars().collect();
    
    let ans=( (c[0] as u8) -  ( ('a' as u8) - ('A' as u8) ) )as char;
    
    println!("{}",ans);
}