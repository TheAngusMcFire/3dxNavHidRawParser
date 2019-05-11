use std::env;
use std::fs::File;
use std::io::Read;
use std::io;
use std::cmp;
extern crate colored;
use colored::*;

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{BarChart, Block, Borders, Widget};
use tui::Terminal;

fn main() 
{
    let arguments : Vec<String> = env::args().collect();
    if arguments.len() != 2 { panic!("Error invalid amount of arguments"); }
    let file_name : &String = &arguments[1];
    println!("{} {}","Using file:".blue(), file_name.green());
    let mut file = File::open(file_name).unwrap();

    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    let mut vector : Vec<(&str, u64)> = vec!
    [
        ("X"    , 9 ),
        ("Y"    , 12),
        ("Z"    , 5 ),
        ("Pitch", 8 ),
        ("Roll" , 2 ),
        ("Yaw"  , 4 )
    ];

    let mut redraw : u64 = 0;
    let mut buffer = [0u8;7];
    let mut parsed_data : ParsedData = ParsedData::new();

    loop
    {
        vector[0].1 = cmp::max(0,400i16 + parsed_data.x    ) as u64;
        vector[1].1 = cmp::max(0,400i16 + parsed_data.y    ) as u64;
        vector[2].1 = cmp::max(0,400i16 + parsed_data.z    ) as u64;
        vector[3].1 = cmp::max(0,400i16 + parsed_data.pitch) as u64;
        vector[4].1 = cmp::max(0,400i16 + parsed_data.roll ) as u64;
        vector[5].1 = cmp::max(0,400i16 + parsed_data.yaw  ) as u64;

        if redraw % 5 == 0
        {
            terminal.draw(|mut f| 
            {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
                    .split(f.size());

                BarChart::default()
                    .block(Block::default().title(" Space mouse values (press mouse btn1 to quit)").borders(Borders::ALL))
                    .data(&vector)
                    .bar_width((f.size().width as f32 / 6.319) as u16)
                    .max(800)
                    .style(Style::default().fg(Color::Red))
                    .value_style(Style::default().fg(Color::Black).bg(Color::Green))
                    .render(&mut f, chunks[0]);
            }).unwrap();
        }

        redraw += 1;

        read_data_from_space_mouse(&mut file, &mut buffer, &mut parsed_data);

        //println!("{:5?}", parsed_data);

        if parsed_data.btn1 {return;}
    }
}


#[derive(Debug, Clone)]
struct ParsedData
{
    x     : i16 ,
    y     : i16 ,
    z     : i16 ,
    pitch : i16 ,
    roll  : i16 ,
    yaw   : i16 ,
    btn1  : bool,
    btn2  : bool
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

fn read_data_from_space_mouse(file : &mut File, buffer : &mut [u8], parsed_data : &mut ParsedData)
{
    let num_bytes = file.read(buffer).unwrap();
    
    if buffer[0] == 1 || buffer[0] == 2
    {assert_eq!(num_bytes,7);}
    else if  buffer[0] == 3
    {assert_eq!(num_bytes,3);}
    else
    {panic!("invalid segment frame descriptor");}        

    parse_hid_frame(&mut buffer[..], parsed_data);
}

fn parse_hid_frame(in_data : &mut [u8], parsed_data : &mut ParsedData)
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
