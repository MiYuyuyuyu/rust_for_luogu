fn p1422() {
    let mut s_temp=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("in s_temp err");
    let ele_num:i32=s_temp.trim().parse().expect("in num err");
// 月用电量在 150 千瓦时及以下部分按每千瓦时 0.4463 元执行
// 月用电量在 151∼400千瓦时的部分按每千瓦时 0.4663 元执行
// 月用电量在 401 千瓦时及以上部分按每千瓦时 0.5663 元执行
    let cost:f64;
    if ele_num <= 150 {
        cost=0.4463*(ele_num as f64);
    }
    else if ele_num <= 400 {
        cost=0.4663*(ele_num -150)as f64 + 0.4463*150.0;
    }
    else{
        cost=0.5663 * (ele_num-400)as f64 + 0.4663*(400-150)as f64 + 0.4463*150.0
    }
    println!("{:.1}",cost);
}