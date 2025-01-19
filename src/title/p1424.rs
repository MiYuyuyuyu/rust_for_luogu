const SWIM:i32=250;
fn main() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let s:Vec<&str>=s_temp.trim().split_whitespace().collect();
    let mut x:i32=s[0].parse().expect("err");
    let n:i32=s[1].parse().expect("err");
    let mut ans=0;
    for i in x..x+n{
        if i%7==6 || i%7==0{
            continue;
        }
        ans+=250;
    }
    println!("{}",ans);
}