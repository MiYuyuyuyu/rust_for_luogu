fn p5724() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let mut a:Vec<i32>=c.iter().map(|i| i.parse().expect("err")).collect();
    a.sort();
    println!("{}",a[a.len()-1]-a[0]);
}