use std::collections::HashMap;

fn p1554() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let a=c[0].parse::<i32>()?;
    let b=c[1].parse::<i32>()?;
    let mut mp:HashMap<char,i32>=HashMap::new();
    for i in a..b+1{
        let temp:String=i.to_string();
        for c in temp.chars(){
            let k=mp.entry(c).or_insert(0);
            *k += 1;
        }
    }
    for i in 0..=9 {
        let k=mp.entry((b'0'+i) as char ).or_insert(0);
        print!("{} ",k);
    }
    println!();
    Ok(())
}