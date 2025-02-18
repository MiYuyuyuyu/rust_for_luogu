fn p1914() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let n=s_temp.trim().parse::<u8>()?;
    let n=n%26;
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp)?;
    let c:Vec<char>=s_temp.trim().chars().collect();
    let ans:Vec<char>=c.iter().map(|&i| {
        let mut u:u8 = i as u8 + n;
        if u>b'z'{
            u-=26;
        }
        u as char
    }).collect();
    ans.iter().for_each(|i| print!("{}",i));
    Ok(())
}