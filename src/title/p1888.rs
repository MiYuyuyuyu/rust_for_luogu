fn gcd(mut a:i32 , mut b:i32)->i32{
    let mut temp:i32;
    loop {
        temp=a;
        a=b;
        b=temp%b;
        if b==0 {
            break;
        }
    }
    a
}

fn p1888() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let abc:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let mut abc:Vec<i32>=abc.iter().map(|i| i.parse::<i32>().expect("err")).collect();
    abc.sort();
    let gcb_num=gcd(abc[0],abc[2]);
    println!("{}/{}",abc[0]/gcd_num,abc[2]/gcd_num);
}