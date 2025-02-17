fn p2615() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_temp = String::new();
    std::io::stdin().read_line(&mut s_temp)?;
    let n:usize=s_temp.trim().parse()?;
    let mut vec_num:Vec< Vec<i32> >=vec![vec![0;n];n];   
    vec_num[0][n/2]=1; 
    let mut temp_x=0_usize;
    let mut temp_y=n/2;
    for i in 2..n.pow(2)+1{
        if temp_x==0 && temp_y !=(n-1) {
            temp_x=n-1;
            temp_y+=1;
        }
        else if temp_y==(n-1) && temp_x!=0 {
            temp_y=0;
            temp_x-=1;
        }
        else if temp_x==0 && temp_y==n-1 {
            temp_x+=1;
        }
        else if temp_x!=0 && temp_y!=n-1  {
            if vec_num[temp_x-1][temp_y+1]==0{
                temp_x=temp_x-1;
                temp_y=temp_y+1;
            }
            else{
                temp_x+=1;
            }
        }
        vec_num[temp_x][temp_y]=i as i32;
    }
    for i in 0..n{
        for j in 0..n{
            print!("{} ",vec_num[i][j]);
        }
        println!("");
    }
    Ok(())
}