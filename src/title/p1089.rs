fn p1089() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    //300
    let mut all_money=0;
    let mut save_money=0;
    for i in 1..=12{
        all_money+=300;
        s_temp.clear();
        std::io::stdin().read_line(&mut s_temp)?;
        let temp=s_temp.trim().parse::<i32>()?;
        if all_money - temp < 0{
            println!("-{}",i);
            return Ok(());
        }
        else{
            all_money-=temp;
            let t=all_money/100;
            save_money+=t*100;
            all_money-=t*100;
        }
    } 
    println!("{}",((save_money as f64)*1.2) as i32 + all_money);
    Ok(())
}