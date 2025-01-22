fn main() {
    let mut s_temp=String::new();
    std::io::stdin().read_line(&mut s_temp).expect("err");
    let k:i32=s_temp.trim().parse().expect("err");
    let mut i=0;
    let mut cnt = 1;
    let mut to_cnt=0;
    let mut ans=0;
    loop{
        if i == k {
            break;
        }
        i+=1;
        if to_cnt<cnt {
            ans+=cnt;
            to_cnt+=1;
        }
        else{
            to_cnt=1;
            cnt+=1;
            ans+=cnt;
        }
    }
    println!("{}",ans);
}