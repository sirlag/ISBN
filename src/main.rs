use std::rand;
use std::os;

type ISBN = Vec<u8>;

fn main(){
    
    let args = os::args();
    match args.len() {
         3 => {
            let s = &args[1];
            match s.as_slice(){

                "-v" => {
                    let parsed_isbn = read_isbn(&args[2]);           
                    match parsed_isbn {
                            Some(isbn) => { println!("Valid: {}", validate(isbn)); },
                            _          => { println!("Failed to parse ISBN"); }
                    };
                },
                "-g" => {
                    match args[2].as_slice() {
                        "10" => generate(10),
                        "13" => generate(13),
                        _  => println!("Please generate your ISBN with 10 or 13 digits")
                    };
                },
                _ => {}
            };
        },

        _ => {
            println!("Usage: {}", args[0]);
            println!("Flags -v, -g");
            println!("\t-v : Usage -v <ISBN>\t-\tVerifies the ISBN.");
            println!("\t-g : Usage -g <size>\t-\tGenerates a new ISBN.");
         }
    };
}

fn read_isbn(s: &String) -> Option<ISBN> {
    let mut isbn = vec![];

    for token in s.chars() {
        match token{
            '0'...'9' => {isbn.push(token as u8 - '0' as u8);},
            'X'       => {isbn.push(10);},
            '-'       => {},
            _         => {return None;}
        };
    }
    if isbn.len() == 10 || isbn.len() == 13 {return Some(isbn);};
    return None;
    
}

fn validate(test : ISBN ) -> bool {
    let mut sum = 0;
    let check_value = if test.len() == 10 { 11} else {10};
    
    for x in 0..test.len(){
        sum += (test[x] * (test.len() - x) as u8) as u16;
        println!("{}", sum);
    }
    println!("{}", sum % check_value);
    return sum % check_value == 0;
}

fn generate(length : u8){
    
    let check_value = if length == 10 { 11} else {10};
    
    let mut new_isbn = vec![];
    let mut to_validate = 0;
    
    for mut x in (0..length-1){
        let temp  = rand::random::<u8>() % 10; 
        to_validate += (temp*(length-x)) as u16;
        new_isbn.push((temp + 48) as char);
    }
    
    let to_match = (to_validate % check_value as u16) as u8;
    
    match 11 - to_match {
        0...9    => {new_isbn.push(((check_value - to_match) + 48) as char)},
        10      => {new_isbn.push('X');},
        11      => {new_isbn.push('0');},
        _       => {println!("You Shouldn't be here...");}
    };
    
    print!("Your new ISBN is : ");
    
    
    for c in new_isbn.iter() {print!("{}", c);}
        
}