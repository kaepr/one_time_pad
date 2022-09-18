use rand::prelude::*;
use std::io;
use std::string::FromUtf8Error;

fn generate_random_bitstream(mut length: usize) -> Vec<u8> {
    let mut generated_string = Vec::new();
    let mut rng = thread_rng();

    while length > 0 {
        let num = rng.gen_range(0..=255);
        generated_string.push(num);
        length -= 1;
    }

    generated_string
}

fn main() -> Result<(), FromUtf8Error> {
    println!("One Time Pad");
    let mut input = String::new();

    print!("Enter message 1:\n");

    io::stdin()
        .read_line(&mut input)
        .expect("error: failed to read user input");

    let msg_1 = input.clone();

    input.clear();
    print!("Enter message 2:\n");

    io::stdin()
        .read_line(&mut input)
        .expect("error: failed to read user input");

    let msg_2 = input.clone();

    let msg_len = msg_1.len();

    if !msg_1.is_ascii() {
        panic!("Message inputted does not follow ASCII standard");
    }

    if !msg_2.is_ascii() {
        panic!("Message inputted does not follow ASCII standard");
    }

    if msg_1.len() != msg_2.len() {
        panic!("Messages are not of same same length");
    }

    println!("msgs: {} {}", msg_1, msg_2);

    let key = generate_random_bitstream(msg_1.len());

    let cipher_text_1: Vec<u8> = msg_1
        .as_bytes()
        .iter()
        .zip(key.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();

    let cipher_text_2: Vec<u8> = msg_2
        .as_bytes()
        .iter()
        .zip(key.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();

    let xorr_cipher: Vec<u8> = cipher_text_1
        .iter()
        .zip(cipher_text_2.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();

    println!("Key Used : {}", String::from_utf8_lossy(&key).to_string());

    println!(
        "Cipher Text 1 : {}",
        String::from_utf8_lossy(&cipher_text_1).to_string()
    );

    println!(
        "Cipher Text 2 : {}",
        String::from_utf8_lossy(&cipher_text_2).to_string()
    );

    loop {
        input.clear();

        print!("Enter a guess word or Ctrl-C to exit:\n");
        io::stdin()
            .read_line(&mut input)
            .expect("error: failed to read user input");

        let guess = input.clone();

        if !guess.is_ascii() {
            println!("Please enter valid ASCII text. Try again.");
            continue;
        }

        println!("");

        let substrings_poss = msg_len - guess.len() + 1;

        for to_skip in 0..substrings_poss {
            let extracted_text: Vec<u8> = xorr_cipher
                .iter()
                .skip(to_skip)
                .zip(guess.as_bytes().iter())
                .map(|(&x1, &x2)| x1 ^ x2)
                .collect();

            print!(
                "Xor of guess word and cipher text from position {}: \n {}\n",
                to_skip,
                String::from_utf8_lossy(&extracted_text).to_string()
            );
        }
    }
}
