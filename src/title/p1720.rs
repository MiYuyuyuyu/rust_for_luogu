fn p1720() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let n:f64=s_temp.trim().parse().expect("err");
    let t1=(1.0+5.0_f64.sqrt())/2.0;
    let t2=(1.0-5.0_f64.sqrt())/2.0;
    let f=(t1.powf(n)-t2.powf(n))/(5.0_f64.sqrt());
    println!("{:.2}",f);
}