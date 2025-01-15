fn p5711(){
    let mut s_temp=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("in s_temp err");
    let year:i32=match s_temp.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("s_temp to year err");
            0
        }
    };
    //4 400
    if (year%100!=0 && year%4==0)|| year%400==0 {
        println!("1");
    }
    else{
        println!("0");
    }
}