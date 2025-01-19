fn p1406() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let high_apple:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let high_apple:Vec<i32>=high_apple.iter()
        .map(|s| s.parse::<i32>().expect("err"))
        .collect();
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let catch_high:i32=s_temp.trim().parse::<i32>().expect("err")+30;
    let mut cnt=0;
    high_apple.iter().for_each(|i| if catch_high>=*i {cnt+=1}); 
    println!("{}",cnt);
}