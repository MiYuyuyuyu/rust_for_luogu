fn p4956() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let n=s_temp.trim().parse::<i32>()?;
    let n=n/52;
    let mut k=1;
    let mut x;
    loop{
        let temp=n-21*k;
        if temp%7 == 0 {
            x=temp/7;
            // 重中之重,x是小于100的
            if x>100 {
                k+=1;
                continue;
            }
            else {
                break;
            }
        }
        k+=1;
    }
    println!("{}\n{}",x,k);
    Ok(())
}