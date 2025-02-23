fn p5734() {
    let mut s_temp=String::new();
    let n:usize=loop{
        s_temp.clear();
        std::io::stdin()
            .read_line(&mut s_temp)
            .expect("err");
        match s_temp.trim().parse(){
            Ok(num)=>break num,
            Err(_)=>{
                println!("err for s_temp to n\ntry again");
                continue;
            },
        }
    };
    let mut str_vec:Vec<String>=vec![String::new();n+1];
    for i in 0..=n{
        std::io::stdin()
            .read_line(&mut str_vec[i])
            .expect("err");
        str_vec[i]=str_vec[i].trim().to_string();
    }
    let mut ans=str_vec[0].clone();
    for x in 1..=n{
        let i = &str_vec[x];
        let temp_vec:Vec<&str>=i.split_whitespace().collect();
        match i.chars().next().unwrap() {
            '1' => {
                ans.push_str(temp_vec[1]);
                println!("{}",ans);
            },
            '2'=>{
                let a=temp_vec[1].parse::<usize>().expect("err in to a");
                let b=temp_vec[2].parse::<usize>().expect("err in to b");
                ans=ans[a..(b+a)].to_string();
                println!("{}",ans);
            },
            '3'=>{
                let a=temp_vec[1].parse::<usize>().expect("err in to a");
                let temp_a=&ans[..a];
                let temp_b=&ans[a..];
                ans=format!("{}{}{}",temp_a,temp_vec[2],temp_b);
                println!("{}",ans);
            },
            '4'=>{
                let pat = temp_vec[1];
                match ans.find(pat) {
                    Some(i)=>{
                        println!("{}",i);
                    }
                    None=>{
                        println!("-1");
                    }
                };
            },
            _=>{

            },
        }
    }

}