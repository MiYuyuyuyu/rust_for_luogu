fn p5725() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let x:i32=s_temp.trim().parse().expect("err");
    let mut s_for_print:String=String::new();
    let mut temp_num=1;
    for _ in 0..x {
        for _ in 0..x {
            let temp=format!("{:02}",temp_num);
            temp_num+=1;
            s_for_print.push_str(&temp);
        }
        s_for_print.push('\n');
    }
    s_for_print.push('\n');
    temp_num=1;
    let mut xx=x;
    for _ in 0..x {
        for _ in 0..xx-1{
            s_for_print.push(' ');
            s_for_print.push(' ');
        }
        xx-=1;
        for _ in 0..(x-xx) {
            let temp=format!("{:02}",temp_num);
            temp_num+=1;
            s_for_print.push_str(&temp);
        }
        s_for_print.push('\n');
    }
    print!("{}",s_for_print);
}