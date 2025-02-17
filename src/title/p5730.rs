fn p5730() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let n:usize=s_temp.trim().parse()?;
    let mut i=0_usize;
    let mut ans:Vec< Vec<char> >=vec![ vec![] ;5 ];
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp)?;
    for c in s_temp.trim().chars(){
        if c=='0'{
            ans[0].push('X');
            ans[0].push('X');
            ans[0].push('X');
            ans[1].push('X');
            ans[1].push('.');
            ans[1].push('X');
            ans[2].push('X');
            ans[2].push('.');
            ans[2].push('X');
            ans[3].push('X');
            ans[3].push('.');
            ans[3].push('X');
            ans[4].push('X');
            ans[4].push('X');
            ans[4].push('X');
        }
        else if c=='1' {
            ans[0].push('.');
            ans[0].push('.');
            ans[0].push('X');
            ans[1].push('.');
            ans[1].push('.');
            ans[1].push('X');
            ans[2].push('.');
            ans[2].push('.');
            ans[2].push('X');
            ans[3].push('.');
            ans[3].push('.');
            ans[3].push('X');
            ans[4].push('.');
            ans[4].push('.');
            ans[4].push('X');
        }
        else if c=='2'{
            ans[0].push('X');
            ans[0].push('X');
            ans[0].push('X');
            ans[1].push('.');
            ans[1].push('.');
            ans[1].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[3].push('X');
            ans[3].push('.');
            ans[3].push('.');
            ans[4].push('X');
            ans[4].push('X');
            ans[4].push('X');
        }
        else if c=='3' {
            ans[0].push('X');
            ans[0].push('X');
            ans[0].push('X');
            ans[1].push('.');
            ans[1].push('.');
            ans[1].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[3].push('.');
            ans[3].push('.');
            ans[3].push('X');
            ans[4].push('X');
            ans[4].push('X');
            ans[4].push('X');
        }
        else if c=='4' {
            ans[0].push('X');
            ans[0].push('.');
            ans[0].push('X');
            ans[1].push('X');
            ans[1].push('.');
            ans[1].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[3].push('.');
            ans[3].push('.');
            ans[3].push('X');
            ans[4].push('.');
            ans[4].push('.');
            ans[4].push('X');
        }
        else if c=='5' {
            ans[0].push('X');
            ans[0].push('X');
            ans[0].push('X');
            ans[1].push('X');
            ans[1].push('.');
            ans[1].push('.');
            ans[2].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[3].push('.');
            ans[3].push('.');
            ans[3].push('X');
            ans[4].push('X');
            ans[4].push('X');
            ans[4].push('X');
        }
        else if c=='6' {
            ans[0].push('X');
            ans[0].push('X');
            ans[0].push('X');
            ans[1].push('X');
            ans[1].push('.');
            ans[1].push('.');
            ans[2].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[3].push('X');
            ans[3].push('.');
            ans[3].push('X');
            ans[4].push('X');
            ans[4].push('X');
            ans[4].push('X');
        }
        else if c=='7' {
            ans[0].push('X');
            ans[0].push('X');
            ans[0].push('X');
            ans[1].push('.');
            ans[1].push('.');
            ans[1].push('X');
            ans[2].push('.');
            ans[2].push('.');
            ans[2].push('X');
            ans[3].push('.');
            ans[3].push('.');
            ans[3].push('X');
            ans[4].push('.');
            ans[4].push('.');
            ans[4].push('X');
        }
        else if c=='8' {
            ans[0].push('X');
            ans[0].push('X');
            ans[0].push('X');
            ans[1].push('X');
            ans[1].push('.');
            ans[1].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[3].push('X');
            ans[3].push('.');
            ans[3].push('X');
            ans[4].push('X');
            ans[4].push('X');
            ans[4].push('X');
        }
        else if c=='9' {
            ans[0].push('X');
            ans[0].push('X');
            ans[0].push('X');
            ans[1].push('X');
            ans[1].push('.');
            ans[1].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[2].push('X');
            ans[3].push('.');
            ans[3].push('.');
            ans[3].push('X');
            ans[4].push('X');
            ans[4].push('X');
            ans[4].push('X');
        }
        i+=1;
        if i!=n {
            ans[0].push('.');
            ans[1].push('.');
            ans[2].push('.');
            ans[3].push('.');
            ans[4].push('.');
        }

    }
    for i in ans{
        for j in i{
            print!("{}",j);
        }
        println!();
    }
    Ok(())
}