use std::cmp::max;
fn p1420() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    s_temp.clear();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let a:Vec<i32>=c.iter().map(|i| i.parse().expect("err")).collect();
    if a.len()==1 {
        println!("0");
        return;
    }
    let mut cnt=0;
    let mut max_cnt=1;
    let mut temp=a[0];
    for i in 1..a.len() {
        if a[i] == temp+1{
            cnt+=1;
            max_cnt=max(max_cnt,cnt);
        }
        else{
            cnt=1;
        }
        temp=a[i];
    }
    println!("{}",max_cnt);
}