use std::env;


fn main() {
    println!("{:?}", "start");
    parse_input();

}


fn parse_input(){
    let args: Vec<String> = env::args().collect();

    match args.len(){
        1 => {
            println!("jd1");
        },
        2 => {
            let num = &args[1];
            let number:i32 = match num.parse(){
                Ok(n) =>{
                    n
                },
                Err(_)=>{
                    println!("jd2");
                    return;
                },

            };
            println!("{:?}", number)
        },
        _ => {
            println!("chuj chuj");
        }

    }

}

