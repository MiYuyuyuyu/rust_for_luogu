fn p1423(){
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let where_go:f64=s_temp.trim().parse().expect("err");
    let mut go_day:f64=2.0;
    let day_day_low:f64=0.98;
    let mut day_go:f64 = 0.0;
    let mut cnt=0;
    loop{
        day_go+=go_day;
        cnt+=1;
        if day_go > where_go {
            break;
        }
        go_day*=day_day_low;
    }
    println!("{}",cnt);
}