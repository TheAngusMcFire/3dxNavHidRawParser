use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct ParsedData
{
    x : i16,
    y : i16,
    z : i16,
    pitch : i16,
    roll :  i16,
    yaw :   i16,
    btn1 : bool,
    btn2 : bool
}

impl ParsedData
{
    fn new() -> ParsedData
    {
        return ParsedData
        {
            x     : 0,
            y     : 0,
            z     : 0,
            pitch : 0,
            roll  : 0,
            yaw   : 0,
            btn1 : false,
            btn2 : false
        }
    }
}

fn main() 
{
    let arguments : Vec<String> = env::args().collect();

    if arguments.len() != 2 { panic!("Error invalid amount of arguments"); }
    let file_name : &String = &arguments[1];

    let mut file = File::open(file_name).unwrap();

    let mut buffer = [0u8;7];
    println!("Using file: {}", file_name);
    
    let mut parsed_data : ParsedData = ParsedData::new();

    loop
    {
        let num_bytes = file.read(&mut buffer).unwrap();
        
        if buffer[0] == 1 || buffer[0] == 2
        {assert_eq!(num_bytes,7);}
        else if  buffer[0] == 3
        {assert_eq!(num_bytes,3);}
        else
        {panic!("invalid segment frame descriptor");}        

        parse_and_print(&mut buffer[..], &mut parsed_data);

        println!("{:3?}", parsed_data);
    }
}

fn parse_and_print(in_data : &mut [u8], parsed_data : &mut ParsedData)
{
    if in_data[0] == 1 
    {
        parsed_data.x = get_encoded_value(&mut in_data[1..3]);
        parsed_data.y = get_encoded_value(&mut in_data[3..5]);
        parsed_data.z = get_encoded_value(&mut in_data[5..7]);
    }
    else if in_data[0] == 2
    {
        parsed_data.pitch = get_encoded_value(&mut in_data[1..3]);
        parsed_data.roll  = get_encoded_value(&mut in_data[3..5]);
        parsed_data.yaw   = get_encoded_value(&mut in_data[5..7]);
    }
    else if in_data[0] == 3
    {
        parsed_data.btn1 = in_data[1] & 0b1  != 0;
        parsed_data.btn2 = in_data[1] & 0b10 != 0;
    }
    else
    {
        println!("{:02X?}", in_data);
        panic!("Invalid frame descriptor");
    } 
}

fn get_encoded_value(in_data : &mut [u8]) -> i16
{
    let value : u16 = (in_data[1] as u16) << 8 | (in_data[0] as u16);

    return value as i16; 
}
