fn p5726() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let s_temp:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let mut a:Vec<i32>=s_temp.iter().map(|i| i.parse().expect("err")).collect();
    a.sort();
    let mut ans=0;
    a.iter().for_each(|i| ans+=i);
    ans-= a[0]+a[a.len()-1];
    let ans:f64=( ans as f64 )/( (a.len()-2) as f64);
    println!("{:.2}",ans);
}