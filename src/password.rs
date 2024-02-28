use rand::Rng;

pub struct PasswordGenerator{
    password_strength: PasswordStrength,
    password_length: u32,
}

pub struct Password{
    password: String,
    service: String,
    login: String,

}

pub enum PasswordStrength{
    LowerCase, //lower case letters
    UpperCase, // lower case + upper case letters
    Numbers, // upper + lower case  + numbers
    Symbols //upper + lower + numbers + symbols
}


fn generate_password(pass_params: &mut PasswordGenerator) -> String{

    let mut rng = rand::thread_rng();
    let password_set:&[u8] = match pass_params.password_strength {
        PasswordStrength::LowerCase =>
            b"abcdefghijklmnopqrstuwxyz"
        ,
        PasswordStrength::UpperCase =>
            b"abcdefghijklmnopqrstuwvxyz\
            ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        ,
        PasswordStrength::Numbers =>
            b"abcdefghijklmnopqrstuwvxyz\
            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            0123456789"
        ,
        PasswordStrength::Symbols =>
            b"bcdefghijklmnopqrstuwvxyz\
            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            0123456789!@#$%^&*()_+-={}[]|:;<>,.?/"


    };

    let password: String = (0..pass_params.password_length)
        .map(|_|{
            let n = rng.gen_range(0..password_set.len());
            password_set[n] as char
        })
        .collect();
    return password

}
