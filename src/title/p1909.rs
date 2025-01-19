use std::i32;

fn p1909() {
    let mut s_temp:String=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("in s_temp err 00");
    let need_pen:i32=s_temp.trim().parse().expect("in need_pen err");
    let mut info_pen:[i32;2]=[0,0];
    let mut min_money:i32=i32::MAX;
    for _ in 0..3{
        s_temp.clear();
        std::io::stdin()
            .read_line(&mut s_temp)
            .expect("in s_temp err 0{i}");
        let c_temp:Vec<&str>=s_temp.trim().split_whitespace().collect();
        info_pen[0]=c_temp[0].parse().expect("err");
        info_pen[1]=c_temp[1].parse().expect("err");
        let cost_pen_times=(need_pen+info_pen[0]-1)/info_pen[0];
        if cost_pen_times*info_pen[1]<min_money{
            min_money=cost_pen_times*info_pen[1];
        }
    }
    println!("{}",min_money);
}