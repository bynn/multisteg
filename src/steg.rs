use std::fs;
use std::io;
use std::io::prelude::*;


// fn main() -> io::Result<()> {
//     let args: Vec<String> = env::args().collect();
//     if args.len() == 1 { panic!("needs more arguments"); }
//     let mut bytes = match read_byte_by_byte(&args[1]) {
//         Ok(line) => line,
//         Err(_) => panic!("Error reading the file {}", &args[1]),
//     }; //bytes is Vec<u8>;
//     let head = heading(&bytes);
//     // println!("{}", head);
//
//     match args.len() {
//         2 => {
//             // eprintln!("number of bytes read: {}", bytes.len());
//             decode(bytes[head..].to_vec());
//         },
//         3 => {
//             // let mut bytes = read_byte_by_byte(&args[1])?; //bytes is Vec<u8>;
//             let msg = fs::read_to_string(&args[2]).expect("Something went wrong reading message");
//             // let msg: String = args[2].trim().parse().expect("second arg should be string");
//             if ((msg.len() * 8) + head) > bytes.len() { panic!("message too long"); }
//             encode(&mut bytes, msg, head);
//             // decode(bytes[13..].to_vec());
//             // eprintln!("number of bytes read: {}", bytes.len());
//             io::stdout().write(&bytes)?;
//             println!("");
//         },
//         _ => println!("wrong number of arguments"),
//     }
//     Ok(())
// }

pub fn heading(bytes: &Vec<u8>) -> usize {
    let mut temp = 0;
    let mut ret: usize = 0;
    for (i, pixel) in bytes.iter().enumerate() {
        if *pixel == 10u8 {
            temp += 1;
            if temp == 3 {
                ret = i;
                break
            }
            // println!("{}, {}", i, *pixel as char);
        }
    }
    ret + 1
}

pub fn decode(bytes: Vec<u8>) -> String {
    let mut show: Vec<char> = Vec::new();
    let mut temp: u8 = 0;
    for (i, pixel) in bytes.iter().enumerate() {
        if i % 8 == 0 && i != 0{
            if temp == 0 { break; }
            show.push(temp as char);
            temp = 0;
        }
        if pixel % 2 == 0 {temp <<= 1}
        else {
            temp <<= 1;
            temp += 1;
        }
    }
    if temp != 0 { panic!("message not found!"); }
    show.push(temp as char);
    let s: String = show.into_iter().collect();
    return s
}

pub fn encode(bytes: &mut Vec<u8>, msg: String, head: usize) -> Vec<u8> {
    let show: Vec<u8> = msg.into_bytes();
    let mut pixel = head;
    for letter in &show {
        for i in (0..8).rev() {
            if (letter & (1 << i)) == 0 { bytes[pixel] = unset_bit(bytes[pixel], 7); }
            else { bytes[pixel] = set_bit(bytes[pixel], 7); }
            pixel += 1;
        }
    }
    for _i in (0..8).rev() {
        bytes[pixel] = unset_bit(bytes[pixel], 7);
        pixel += 1;
    }
    return bytes.to_vec();
    // decode(bytes.to_vec());
    // io::stdout().write(&bytes);
    // println!("{}", msg);
    // println!("{:?}", show);
    // for pixel in bytes[13..37].to_vec() {
    //     println!("{}", pixel);
    // }
}

pub fn read_byte_by_byte(path_to_file: &str) -> Result<Vec<u8>, io::Error> {
    let mut f = fs::File::open(path_to_file)?;
    let mut bytes = vec![0u8; 0];
    let mut byte_buffer = [0u8; 8];

    while f.read(&mut byte_buffer)? != 0 {
        bytes.extend(&byte_buffer);
    }
    Ok(bytes)
}

pub fn set_bit(byte: u8, position: u8) -> u8 {
    match position {
        0 => byte | 0b1000_0000,
        1 => byte | 0b0100_0000,
        2 => byte | 0b0010_0000,
        3 => byte | 0b0001_0000,
        4 => byte | 0b0000_1000,
        5 => byte | 0b0000_0100,
        6 => byte | 0b0000_0010,
        7 => byte | 0b0000_0001,
        _ => panic!("set panic!"),
    }
}

pub fn unset_bit(byte: u8, position: u8) -> u8 {
    match position {
        0 => byte & 0b0111_1111,
        1 => byte & 0b1011_1111,
        2 => byte & 0b1101_1111,
        3 => byte & 0b1110_1111,
        4 => byte & 0b1111_0111,
        5 => byte & 0b1111_1011,
        6 => byte & 0b1111_1101,
        7 => byte & 0b1111_1110,
        _ => panic!("unset panic!"),
    }
}