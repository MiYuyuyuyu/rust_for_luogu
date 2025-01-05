fn P2029() {
    //1L=100cm*100cm*100cm;
    let pi=3.14;
    let mut s=String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("s的填充报错");
    let ss:Vec<&str>=s.trim().split_whitespace().collect();
    let h:f64=ss[0].parse().expect("h的填充报错");
    let r:f64=ss[1].parse().expect("r的填充报错");
    let v:f64=pi * r * r * h;
    let tt:f64=(20000.0+v-1.0)/v;
    let t=tt as i32;
    println!("{}",t);
}