use std::env;
use rand::Rng;

fn main() {
    println!("{:?}", "start");
    parse_input();
    generate_password(4);


}


fn generate_password(pass_len: i32) {
    let mut rng = rand::thread_rng();
    let mut password: Vec<char> = Vec::new();
    //make it the "rust-way"
    for x in 0..pass_len{
        match char::from_u32(rng.gen_range(65..90)){
            Some(n) =>{
                password.push(n);
            },
            None =>{
                println!("jdjd");
            }
        }

    }
    println!("{:?}",password)
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

