fn b2025() {
    //打5行,1,3,5,3,1
    let x=5;
    let mut f=1;
    let mut bol =1;
    let mut x_x;
    for i in 0..x{

        x_x = match f {
            1=>2,
            3=>1,
            5=>0,
            _ =>{
                println!("报错");
                return;
            }
        };

        for _ in 0..x_x{
            print!(" ");
        }

        for _ in 0..f{
            print!("*");
        }
        println!("");

        if i==2 {
            bol=-1;
        }

        f += 2*bol;
    }
}