fn zs_bool(a:i64)->bool {
    for i in 2..a{
        if a%i == 0{
            return false
        }
    }
    true
}

fn p5723() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let n:i64=s_temp.trim().parse().expect("err");
    let mut cnt_num=0;
    let mut sum=0;
    let mut i=2;
    loop {
        if zs_bool(i) {
            let temp_sum=sum+i;
            if temp_sum<=n{
                sum+=i;
                cnt_num+=1;
                println!("{}",i);
                // println!("{}",sum);
            }
            else{
                break;
            }
        }
        i+=1;
    }
    println!("{}",cnt_num);
}