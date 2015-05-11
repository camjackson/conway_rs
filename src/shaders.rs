use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub fn load(name: &str) -> String {
    let path = format!("shaders/{}.glsl", name);
    let mut file = match File::open(&Path::new(&path)) {
        Err(why) => panic!("Could not open shader '{}': {}", path, why),
        Ok(file) => file
    };

    let mut shader_text = String::new();
    match file.read_to_string(&mut shader_text) {
        Err(why) => panic!("Could not read vertex shader: {}", why),
        Ok(_) => shader_text
    }
}
