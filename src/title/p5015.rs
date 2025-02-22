fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let c_vec:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let mut sum=0;
    c_vec.iter().for_each(|i| sum+=i.len());
    println!("{}",sum);
    Ok(())
}