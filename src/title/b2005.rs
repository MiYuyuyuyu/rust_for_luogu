fn b2005() {
    let mut s=String::new();
    std::io::stdin()
        .read_line(& mut s)
        .expect("s的填充报错");
    let x=s.trim();
    println!("  {}\n {}{}{}\n{}{}{}{}{}",x,x,x,x,x,x,x,x,x);
}