mod steg;

use steg::*;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::thread;
use std::cmp;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::collections::HashMap;



fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 { panic!("needs more arguments"); }
    println!("{:?}", args);
    let numthread: u8 = args[1].parse::<u8>().unwrap();
    println!("number of threads: {}", numthread);
    // let mut bytes = match read_byte_by_byte(&args[2]) {
    //     Ok(line) => line,
    //     Err(_) => panic!("Error reading the file {}", &args[2]),
    // }; //bytes is Vec<u8>;
    // let head = heading(&bytes);
    // println!("{}", head);

    match args.len() {
        3 => {
            println!("** DECODE ***");
            let dir = &args[2];
            let mut names: Vec<String> = read_directory(dir);
            names.sort();

            let threadsize = cmp::min(numthread, names.len() as u8);

            let mut pairs: Vec<(usize, String)> = Vec::new();

            for (i, name) in names.iter().enumerate() {
                pairs.push((i, name.to_string()));
            }
            pairs.sort();

            let mut handles = vec![];
            let mut ret: Vec<(usize, String)> = vec![];

            let counter = Arc::new(Mutex::new(0_usize));

            let (tx, rx) = mpsc::channel();

            for x in 1..=threadsize {
                let (counter, tx) = (Arc::clone(&counter), tx.clone());
                let pairs = pairs.clone();

                let handle = thread::spawn(move || {
                    println!("spawning thread #{}", x);
                    let mut counter = counter.lock().unwrap();
                    let pair = &pairs[*counter];
                    println!("{:?}", pair);
                    *counter += 1;
                        // while pair.0 < pairs.len() {
                        println!("decoding {:?} on thread #{}", pair, x);
                        let bytes = match read_byte_by_byte(&pair.1) {
                            Ok(line) => line,
                            Err(_) => panic!("Error reading the file {}", &pair.1),
                        }; //bytes is Vec<u8>;
                        let head = heading(&bytes);
                        let s = decode(bytes[head..].to_vec());
                        tx.send((pair.0,s)).unwrap();
                        // }

                });
                handles.push(handle);
            }

            let mut got = 0;
            for received in &rx {
                println!("got: {}", received.0);
                ret.push(received);
                got += 1;
                if got == pairs.len() { break };
            }

            for handle in handles {
                handle.join().unwrap();
            }

            ret.sort();
            for retval in &ret {
                println!("{}", retval.1);
            }
        },
        5 => {
            println!("*** ENCODE ***");

            let mut msg = fs::read_to_string(&args[2]).expect("Something went wrong reading message");
            println!("meg.len() = {}", msg.len());

            let names: Vec<String> = read_directory(&args[3]);
            println!("found {} valid ppm files in {}", names.len(), &args[3]);
            let mut capacity: HashMap<String, usize> = HashMap::new();
            for name in &names {
                let bytes = match read_byte_by_byte(name) {
                    Ok(line) => line,
                    Err(_) => panic!("Error reading the file {}", name),
                }; //bytes is Vec<u8>;
                let head = heading(&bytes);
                let cap = (bytes.len() - head)/8;
                capacity.insert(name.to_string(), cap - 1);
            }
            // for name in &names {
            //     println!("{}: {}", name, capacity[name]);
            // }
            let mut plans: Vec<(usize, String, String)> = vec![];
            let mut cyc = names.iter().cycle();
            let mut count = 0usize;
            while msg.len() > 0 {
                let filename = cyc.next().unwrap();
                let slice: String;
                if capacity[filename] < msg.len() {
                    slice = msg[..capacity[filename]].to_string();
                    msg = msg[capacity[filename]..].to_string();
                }
                else {
                    slice = msg;
                    msg = "".to_string();
                }
                plans.push((count, filename.to_string(), slice));
                count += 1;
            }

            let counter = Arc::new(Mutex::new(0_usize));
            let (tx, rx) = mpsc::channel();
            let threadsize = cmp::min(numthread, plans.len() as u8);
            let mut handles = vec![];

            for x in 1..=threadsize {
                let plans = plans.clone();
                let (counter, tx) = (Arc::clone(&counter), tx.clone());

                let handle = thread::spawn(move || {
                    println!("spawning thread #{}", x);
                    let mut counter = counter.lock().unwrap();
                    let plan = &plans[*counter];
                    *counter += 1;
                    println!("encoding on thread #{}", x);
                    let mut bytes = match read_byte_by_byte(&plan.1) {
                        Ok(line) => line,
                        Err(_) => panic!("Error reading the file {}", &plan.1),
                    }; //bytes is Vec<u8>;
                    let count = plan.0 as usize;
                    let head = heading(&bytes);
                    let msg = plan.2.to_string();
                    let ppm = encode(&mut bytes, msg, head);
                    tx.send((count, ppm)).unwrap();
                });
                handles.push(handle);
            }

            fs::create_dir(&args[4])?;
            let mut got = 0;
            for received in &rx {
                let txt = format!("{}/{:010}.ppm", &args[4], received.0);
                println!("got: {}", txt);
                got += 1;
                let mut file = fs::File::create(txt)?;
                file.write_all(&received.1)?;
                if got == 10 { break };
            }

            for handle in handles {
                handle.join().unwrap();
            }
        },
        _ => println!("wrong number of arguments"),
    }
    Ok(())
}


// referenced from stack overflow
fn read_directory(dir: &String) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();

    let names = paths
    .map(|entry| {
        let entry = entry.unwrap();

        let entry_path = entry.path();

        let file_name = entry_path.file_name().unwrap();

        let file_name_as_str = file_name.to_str().unwrap();

        let file_name_as_string = String::from(file_name_as_str);

        format!("{}/{}", dir, file_name_as_string)
    })
    .filter(|file_name_as_string| file_name_as_string.ends_with(".ppm"))
    .collect::<Vec<String>>();

    return names;
}