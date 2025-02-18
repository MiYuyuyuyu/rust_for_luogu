fn p() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    println!("{}",s_temp.trim().to_uppercase() );
    Ok(())
}