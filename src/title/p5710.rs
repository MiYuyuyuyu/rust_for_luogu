fn p5710() {
    // i
    // %2 >4 <=12
    let mut s_temp:String=String::new();
    std::io::stdin()
        .read_line(& mut s_temp)
        .expect("in s_temp err");
    let num_x:i32= match s_temp.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("in num_x err");
            12
        }
    };
    let b1:bool=num_x%2==0;
    let b2:bool=(num_x > 4)&&(num_x <=12);
    let mut p:Vec<i32>=Vec::new();
    if b1&&b2{
        p.push(1);
    }
    else{
        p.push(0);
    }
    if b1||b2{
        p.push(1);
    }
    else{
        p.push(0);
    }

    if (b1&&!b2)||(!b1&&b2) {
        p.push(1);
    }
    else{
        p.push(0);
    }

    if !b1&&!b2{
        p.push(1);
    }
    else{
        p.push(0);
    }
    for i in p{
        print!("{} ",i);
    }
    println!("");
}