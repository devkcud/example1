use std::{fs::File, io::Read, env::{args, current_exe}, process::exit, path::Path};

fn main() {
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    if args.len() == 0 {
        println!("Usage: {} <file>", current_exe().unwrap().to_str().unwrap().split('/').collect::<Vec<&str>>().last().unwrap());
        exit(1);
    }
    
    if !Path::new(args.get(0).unwrap()).exists() {
        println!("File {} doesn't exist.", args.get(0).unwrap());
        exit(1);
    }

    let mut file:   File = File::open(args.get(0).unwrap()).expect(format!("Error opening file: {}", args.get(0).unwrap()).as_str());
    let mut data: String = String::new();

    file.read_to_string(&mut data).expect("Err");

    const ARRAY_SIZE: usize = 32;
    const INDEX_MIN:  usize = 0;
    const INDEX_MAX:  usize = ARRAY_SIZE - 1;
    const BYTE_MIN:     u16 = 0;
    const BYTE_MAX:     u16 = 255; // Hard limit because of limitations in my code and I am too lazy to create tests. My bad... Hours spent: 2

    let arr:             [u16; ARRAY_SIZE] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut byte_list:   [u16; ARRAY_SIZE] = arr;
    let mut c_byte_list: [u16; ARRAY_SIZE] = arr;

    let mut index: usize = INDEX_MIN;

    for line in data.lines() {
        for char in line.chars().collect::<Vec<char>>().iter() {
            match char {
                ')'  => byte_list[index] = byte_list[index].wrapping_add(  1).min(BYTE_MAX),
                ']'  => byte_list[index] = byte_list[index].wrapping_add( 10).min(BYTE_MAX),
                '}'  => byte_list[index] = byte_list[index].wrapping_add(100).min(BYTE_MAX),

                '('  => byte_list[index] = byte_list[index].wrapping_sub(  1).max(BYTE_MIN),
                '['  => byte_list[index] = byte_list[index].wrapping_sub( 10).max(BYTE_MIN),
                '{'  => byte_list[index] = byte_list[index].wrapping_sub(100).max(BYTE_MIN),

                '>'  => index = index.wrapping_add(1).min(INDEX_MAX),
                '<'  => index = index.wrapping_sub(1).max(INDEX_MIN),

                '\'' => c_byte_list.clone_from(&byte_list),
                '"'  => byte_list.clone_from(&c_byte_list),

                '='  => println!("{}", String::from_utf16(&byte_list).unwrap()),
                '#'  => println!("{:?}", &byte_list),

                '&'  => println!("{}", index), // Show index
                '@'  => println!("{}", byte_list[index]), // Show hold value

                '!'  => byte_list[index] = 0,
                '$'  => byte_list.fill(0),
                '.'  => index = 0,

                '~'  => arr.into_iter().enumerate().for_each(|(i, _c)| c_byte_list[i] = byte_list[INDEX_MAX - i]),

                _    => {
                    // TODO: Make goto up to 31 (maximum)
                    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&char).then(|| index = usize::try_from(char.to_digit(10).unwrap()).unwrap());
                },
            }
        }
        
        if !line.starts_with("`") { continue; }
        
        let line = line.replace("`", "");
        let mut command: Vec<&str> = line.split_whitespace().collect();
        
        match command.remove(0) {
            "swap" => {
                if command.len() < 2 { continue; }

                let from = command.get(0).unwrap().to_string().parse::<usize>();
                let to   = command.get(1).unwrap().to_string().parse::<usize>();
                
                match from { Ok(_o) => (), Err(_e) => continue }
                match to   { Ok(_o) => (), Err(_e) => continue }

                let copy_from = from.unwrap().max(INDEX_MIN);
                let copy_to   = to.unwrap().min(INDEX_MAX);

                let cached_value = byte_list[copy_from];

                byte_list[copy_from] = byte_list[copy_to];
                byte_list[copy_to]   = cached_value;
            },

            "rem" => {
                if command.len() == 0 { continue; }

                let from = command.get(0).unwrap().to_string().parse::<usize>();
                
                match from { Ok(_o) => (), Err(_e) => continue }
                
                byte_list[from.unwrap().max(INDEX_MIN).min(INDEX_MAX)] = 0;
            },

            _ => (),
        }
    }
}
