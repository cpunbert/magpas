use std::env;
use rand::Rng;

enum PasswordStrength{
    LowerCase, //lower case letters
    UpperCase, // lower case + upper case letters
    Numbers, // upper + lower case  + numbers
    Symbols //upper + lower + numbers + symbols
}


fn main() {
    println!("{:?}", "start");
    println!("{:?}", generate_password(20, PasswordStrength::Symbols));

}


fn generate_password(pass_len: i32,pass_strength: PasswordStrength) -> String{
    const LOWERCASE:  &[u8] = b"abcdefghijklmnopqrstuwxyz";
    const UPPERCASE:  &[u8] = b"abcdefghijklmnopqrstuwvxyz\
                                ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS: &[u8] = b"abcdefghijklmnopqrstuwvxyz\
                            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789";
    const SYMBOLS: &[u8] = b"bcdefghijklmnopqrstuwvxyz\
                            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789!@#$%^&*()_+-={}[]|:;<>,.?/";

    let mut rng = rand::thread_rng();


    let password_set = match pass_strength {
        PasswordStrength::LowerCase =>{
          LOWERCASE
        },
        PasswordStrength::UpperCase =>{
            UPPERCASE
        },
        PasswordStrength::Numbers =>{
            NUMBERS
        },
        PasswordStrength::Symbols =>{
            SYMBOLS
        }

    };

    let password: String = (0..pass_len)
        .map(|_|{
            let n = rng.gen_range(0..password_set.len());
            password_set[n] as char
        })
        .collect();
    return password



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

