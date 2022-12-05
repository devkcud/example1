use std::{fs::File, io::Read, env::{args, current_exe}, process::exit, path::Path};

#[allow(unused_comparisons)]
fn main() {
    let mut index: usize = 0;
    let mut byte_list: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut copy_byte_list: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    
    if args.len() == 0 {
        println!("usage: {} <file>", current_exe().unwrap().to_str().unwrap().split('/').collect::<Vec<&str>>().last().unwrap());
        exit(1);
    }
    
    if !Path::new(args.get(0).unwrap()).exists() {
        println!("File {} doesn't exist.", args.get(0).unwrap());
        exit(1);
    }

    let mut file = File::open(args.get(0).unwrap()).expect(format!("Error opening file: {}", args.get(0).unwrap()).as_str());
    let mut data = String::new();

    file.read_to_string(&mut data).expect("Err");
    let data = data.trim();
    let data = data.replace(" ", "").replace("\n", "");
    
    for command in data.chars() {
        match command {
            '>' => {
                if index < 15 {
                    index += 1;
                }
            },
            '<' => {
                if index > 0 {
                    index -= 1;
                }
            },
            '+' => {
                let add: u8 = 1;

                byte_list[index] = byte_list[index].wrapping_add(add).min(127);
            },
            '-' => {
                let rem: u8 = 1;
                
                byte_list[index] = byte_list[index].wrapping_sub(rem).min(0);
            },
            ']' => {
                let add: u8 = 10;

                byte_list[index] = byte_list[index].wrapping_add(add).min(127);
            },
            '[' => {
                let rem: u8 = 10;

                byte_list[index] = byte_list[index].wrapping_sub(rem).min(0);
            },
            ')' => {
                let add: u8 = 100;

                byte_list[index] = byte_list[index].wrapping_add(add).min(127);
            },
            '(' => {
                let rem: u8 = 100;
                
                byte_list[index] = byte_list[index].wrapping_sub(rem).min(0);
            },
            '@' => println!("{}", String::from_utf8(byte_list.to_vec().clone()).unwrap()),
            '#' => println!("{:?}", byte_list),
            '!' => byte_list[index] = 0,
            '~' => { byte_list.fill(0); index = 0; },
            '?' => println!("{}", index),
            '&' => println!("{}", byte_list[index]),
            '*' => copy_byte_list = byte_list,
            '/' => byte_list = copy_byte_list,
            _ => (),
        }
    }
}
