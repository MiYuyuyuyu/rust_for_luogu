fn p5717() {
    // a,b,c 的长度，均是不大于 
    // 10000
    // 10000 的正整数。打算把这三条线段拼成一个三角形，它可以是什么三角形呢？
        let mut s_temp=String::new();
        std::io::stdin()
            .read_line(&mut s_temp)
            .expect("in s_temp err");
        let c:Vec<&str>=s_temp.trim().split_whitespace().collect();
        let a:i32=c[0].parse().expect("err a");
        let b:i32=c[1].parse().expect("err b");
        let c:i32=c[2].parse().expect("err c");
        let x=std::cmp::max(a, std::cmp::max(b,c));
        let z=std::cmp::min(a, std::cmp::min(b,c));
        let sum=a+b+c;
        let y= sum-x-z;
        //x y z 大->小
        // 如果三条线段不能组成一个三角形，输出Not triangle；
        if y+z <= x{
            println!("Not triangle");
        }
        else {
            // 如果是直角三角形，输出Right triangle；
            if (y*y+z*z)==x*x{
                println!("Right triangle")
            }
    
            // 如果是锐角三角形，输出Acute triangle；
            else if (y*y+z*z)>x*x{
                println!("Acute triangle");
            }
    
            // 如果是钝角三角形，输出Obtuse triangle；
            if (y*y+z*z)<x*x{
                println!("Obtuse triangle");
            }
    
            // 如果是等腰三角形，输出Isosceles triangle；
            if x==y||y==z{
                println!("Isosceles triangle");
            }
    
            // 如果是等边三角形，输出Equilateral triangle。
            if x==y&&y==z{
                println!("Equilateral triangle");
            }
        }
    }