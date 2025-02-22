fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let n:i32=s_temp.trim().parse()?;
    let mut char_temp:char='a';
    //a+ b- c*
    for _ in 0..n {
        s_temp.clear();
        std::io::stdin().read_line(&mut s_temp)?;
        let c= s_temp.trim().chars().next().unwrap();
        let c_vec:Vec<&str>=s_temp.trim().split_whitespace().collect();
        // let c_vec:Vec<i32>=c_vec.iter().map(|i| i.parse::<i32>().expect("err")).collect();
        match c {
            'a'=>{
                let temp=format!("{}+{}={}",c_vec[1],c_vec[2],(c_vec[1].parse::<i32>()?+c_vec[2].parse::<i32>()?));
                println!("{}",temp);
                println!("{}",temp.len());
                char_temp='a';
            },
            'b'=>{
                let temp=format!("{}-{}={}",c_vec[1],c_vec[2],(c_vec[1].parse::<i32>()?-c_vec[2].parse::<i32>()?));
                println!("{}",temp);
                println!("{}",temp.len());
                char_temp='b';
            },
            'c'=>{
                let temp=format!("{}*{}={}",c_vec[1],c_vec[2],(c_vec[1].parse::<i32>()?*c_vec[2].parse::<i32>()?));
                println!("{}",temp);
                println!("{}",temp.len());
                char_temp='c';
            },
            _=>{
                match char_temp {
                    'a'=>{
                        let temp=format!("{}+{}={}",c_vec[0],c_vec[1],(c_vec[1].parse::<i32>()?+c_vec[0].parse::<i32>()?));
                        println!("{}",temp);
                        println!("{}",temp.len());
                    },
                    'b'=>{
                        let temp=format!("{}-{}={}",c_vec[0],c_vec[1],(c_vec[0].parse::<i32>()?-c_vec[1].parse::<i32>()?));
                        println!("{}",temp);
                        println!("{}",temp.len());
                    },
                    'c'=>{
                        let temp=format!("{}*{}={}",c_vec[0],c_vec[1],(c_vec[1].parse::<i32>()?*c_vec[0].parse::<i32>()?));
                        println!("{}",temp);
                        println!("{}",temp.len());
                    },
                    _=>{
                        println!("err");
                    }
                }
            },
        }
    }
    Ok(())
}