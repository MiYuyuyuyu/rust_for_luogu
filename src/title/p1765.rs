fn main() {
    let mut s_temp=String::new();
    std::io::stdin()
        .read_line(&mut s_temp)
        .expect("err");
    let a:[i32;26]=[1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,4,1,2,3,1,2,3,4];
    let mut sum=0;
    for i in s_temp.chars(){
        if i >= 'a' && i <= 'z' {
            sum += a[((i as u8)-b'a')as usize];
        }
        else if i == ' '{
            sum+=1;
        }
        else{
            continue;
        }
    }
    println!("{}",sum);
}