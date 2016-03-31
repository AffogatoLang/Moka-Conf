use std::env;
use std::path::Path;
use std::io;
use std::io::Write;
use std::fs;

pub fn get_pwd() -> String {
    env::var("PWD").unwrap()
}
pub fn get_pwd_name() -> String {
    let pwd = env::var("PWD").unwrap();
    let pwd_path = Path::new(&pwd);
    (&(*(pwd_path.file_name().unwrap()).to_str().unwrap())).to_string()
}

pub fn write_toml_out(filepath:String, toml:String) -> io::Result<()> {
    let path = Path::new(&filepath);
    fs::File::create(path).unwrap().write_all(toml.as_bytes())
}
