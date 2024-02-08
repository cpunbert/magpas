use rand::Rng;
#[derive(Debug, Default)]
pub struct App{
    pub password: String,
    pub exit: bool,
}

pub enum PasswordStrength{
    LowerCase, //lower case letters
    UpperCase, // lower case + upper case letters
    Numbers, // upper + lower case  + numbers
    Symbols //upper + lower + numbers + symbols
}


impl App{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn tick(&self) {}


    pub fn quit(&mut self){
        self.exit = true;
    }

    pub fn gen_pass(&mut self, pass_len: i32, pass_strength: PasswordStrength ){
        if let Some(res) = self.password.generate_password(pass_len, pass_strength){
        self.password = res;
        }
    }


    fn generate_password(pass_len: i32,pass_strength: PasswordStrength) -> String{

        let mut rng = rand::thread_rng();
        let password_set:&[u8] = match pass_strength {
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

        let password: String = (0..pass_len)
            .map(|_|{
                let n = rng.gen_range(0..password_set.len());
                password_set[n] as char
            })
            .collect();
        return password

    }

}