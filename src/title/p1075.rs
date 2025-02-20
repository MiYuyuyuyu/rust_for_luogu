fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(& mut s_temp)?;
    let n=s_temp.trim().parse::<i32>()?;
    let k=(n as f64).sqrt() as i32;
    for i in 2..=k{
        if n%i == 0{
            println!("{}",n/i);
            return Ok(());
        }
    }
    Ok(())
}