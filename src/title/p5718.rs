use std::error::Error;

fn p5718() -> Result<(),Box<dyn Error>> {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp)?;
    let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let ans:Vec<i32>=c.iter().map(|c| c.parse::<i32>().expect("err")).collect();
    if let Some(min_num)=ans.iter().min(){
        println!("{}",min_num);
    }
    Ok(())
}