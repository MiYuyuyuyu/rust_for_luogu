fn p5719() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let str_temp:Vec<&str> = s_temp.trim().split_whitespace().collect();
    let n:i32=str_temp[0].parse().expect("err");
    let k:i32=str_temp[1].parse().expect("err");
    let mut cnt_a: i32=0;
    let mut cnt_b: i32=0;
    let mut ans_a: i32=0;
    let mut ans_b: i32=0;
    for i in 1..n+1 {
        if i%k==0 {
            cnt_a+=1;
            ans_a+=i;
        }
        else{
            cnt_b+=1;
            ans_b+=i;
        }
    }
    let  ans_a: f64 =ans_a as f64/cnt_a as f64;
    let  ans_b: f64 =ans_b as f64/cnt_b as f64;
    println!("{:.1} {:.1}",ans_a,ans_b);
}