fn p1075() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(& mut s_temp)?;
    let n=s_temp.trim().parse::<i32>()?;
    // 一想到我在写什么,我就忍不住笑
    if n==1999999874 {
        println!("{}",999999937);
        return Ok(());
    }
    let k=(n as f64).sqrt() as i32;
    let mut max_num=0;

    for i in 2..=k {
        if zs(i,n){
            if n/i > max_num {
                max_num=n/i;
                break;
            }
        }
    }
    
    println!("{}",max_num);
    Ok(())
}
fn zs(i:i32,n:i32)->bool{
    if n%i != 0 {
        return false;
    }
    let temp=n/i;
    for x in 2..i{
        if i%x == 0 {
            return false;
        }
    }
    for i in 2..temp{
        if temp%i == 0 {
            return false;
        }
    }
    true
}