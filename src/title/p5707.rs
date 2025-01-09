fn p5707() {
    //s v
    let mut s_temp:String=String::new();
    std::io::stdin()
        .read_line(& mut s_temp)
        .expect("in s_temp err");
    let c_temp:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let s:i32=match c_temp[0].parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("c_temp in s err");
            100
        }
    };
    let v:i32=match c_temp[1].parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("c_temp in v err");
            99
        }
    };
    let other_time: i32=10;
    let use_time: i32=( (s+v-1)/v )as i32+other_time;
    let mut cost_hour: i32=( use_time/60 ) as i32;
    let mut cost_min: i32= ( use_time%60 ) as i32;

    cost_hour=cost_hour%24;

    if cost_min!=0 {
        cost_min=60-cost_min;
    }

    if cost_hour>7 {
        cost_hour=24+(7-cost_hour);//当时写道这里写成24 - (7-cost_hour),卡了半小时,悔不当初😅
    }
    else{
        cost_hour=7-cost_hour;
    }


    let ans_s = format!("{:02}:{:02}",cost_hour, cost_min);
    println!("{}", ans_s);
}