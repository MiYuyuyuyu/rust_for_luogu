fn main() {
    //读取字符串
    let mut s_temp=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("err");
    //将字符串以'-'进行分割,存储到切片的动态数组
    let c:Vec<&str>=s_temp.trim().split("-").collect();
    //建立一个新的字符串,把所有动态数组的切片存入一个新字符串
    let mut all_str=String::new();
    c.iter().for_each(|x| all_str.push_str(x));

    //将字符串转换为char数组
    let c:Vec<char>=all_str.chars().collect();
    //对char数组进行处理,将每个字符转换为数字,如果遇到X就转换为10
    let vec_num:Vec<i32>=c.iter().
        map(
            |i| 
            if *i !='X' {
                i.to_string().parse().expect("err")
            } 
            else {
                10
            })
            .collect();
    // 累加
    let mut ans=0;
    for i in 0..vec_num.len()-1 {
        // print!("{}  -->{}\n",i,vec_num[i]);
        ans+=(i as i32 + 1)*vec_num[i];//记得+1 重中之重
    }
    ans=ans%11;
    //相等输出Right
    if ans==vec_num[vec_num.len()-1] {
        println!("Right");
    }
    //不相等的话处理值
    else {
        s_temp=s_temp.trim().to_string();
        s_temp.pop();
        let temp_c=(b'0'+ans as u8)   as char;
        if temp_c as u8 != b'9'+1{
            s_temp.push(temp_c);
        }
        else{
            s_temp.push('X');
        }
        println!("{}",s_temp);
    }
}