use std::collections::HashMap;

fn p2550() -> Result<(), Box<dyn std::error::Error>> {
    //取得 n
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let n:usize=s_temp.trim().parse()?;

    // 建立哈希表存储中奖号码
    let mut ans:HashMap<i32,bool>=HashMap::new();
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp)?;
    let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
    for item in c.iter() {
        let t: i32 = item.parse()?;
        ans.insert(t, true);
    }

    // 循环读取数组号码,判断数字是否出现
    let mut vec_num:Vec<i32>=vec![0;7];
    for _ in 0..n{
        s_temp.clear();
        std::io::stdin().read_line(&mut s_temp)?;
        let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
        let temp:Vec<i32>=c.iter().map(|i| i.parse().expect("err")).collect();
        let mut cnt=0;
        for x in temp {
            let c=ans.entry(x).or_insert(false);
            if *c {
                cnt+=1;
            }
        }
        if cnt !=0{
            vec_num[7-cnt]+=1;
        }
    }

    //打印数组
    for x in vec_num {
        print!("{} ",x);
    }
    println!();
    Ok(())
}