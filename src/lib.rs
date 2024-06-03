pub struct Config {
    pub password: String,
    pub file_path: String,
    pub command: String,
}

pub fn parse_config(args: &[String]) -> Config {
    let password = args[1].clone();
    let file_path = args[2].clone();
    let command = args[3].clone();

    Config { password, file_path, command }
}

pub fn create_key(password:String) -> Vec<u8> {
    let mut b: Vec<u8> = password.as_bytes().to_vec();

    if b.len() < 16 {
        b.resize(32, 0);
    }

    b
} 
