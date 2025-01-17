fn p1085() {
    let mut class_learn:[i32;7]=[0;7];
    let mut class_habit:[i32;7]=[0;7];
    let mut s_temp:String=String::new();
    let mut max_learn_time:i32=0;
    let mut max_learn_day:i32=0;
    for i in 0..7{
        s_temp.clear();
        std::io::stdin()
            .read_line(&mut s_temp)
            .expect("in s_temp err 00");
        let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
        class_learn[i]=c[0].parse().expect("err");
        class_habit[i]=c[1].parse().expect("err");
        let tatal_learn=class_learn[i] + class_habit[i];
        if tatal_learn > 8{
            if tatal_learn > max_learn_time{
                max_learn_time=tatal_learn;
                max_learn_day=(i as i32)+1;
            }
        }
    }
    println!("{}",max_learn_day);
}