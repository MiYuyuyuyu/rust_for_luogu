fn p5716() {
    let mut s_temp:String=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("in s_temp err 00");
    let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let year:i32=match c[0].parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("in year err 00");
            1926
        }
    };
    let month:i32=match c[1].parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("in month err 00");
            8
        }
    };
    let month_day:[i32;12]=[31,28,31,30,31,30,31,31,30,31,30,31];
    let run_year:bool=(year%100!=0&&year%4==0)||year%400==0 ;
    if run_year && month==2 {
        println!("{}",month_day[ (month-1) as usize ]+1);
    }
    else{
        println!("{}",month_day[ (month-1) as usize ]);
    }
}