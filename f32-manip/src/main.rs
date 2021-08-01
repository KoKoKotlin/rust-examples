extern crate ncurses;

use ncurses::*;
use std::{cmp::{min, max}, intrinsics::transmute};

const POINTER: u32 = 2;

fn print_bit_patter(bits: u32, position: i32) {
    let float : f32 = unsafe {
        transmute(bits)
    };

    let offset = 40;
    mvprintw(1, 1, "Bit representation:        ");

    for i in 0..32 {
        if position - 1 == i as i32 {
            mvprintw(2, (i + offset) as i32, String::from('@').as_str());
        }
        
        mvprintw(1, (i + offset) as i32, format!("{}", 1 & (bits >> (31 - i))).as_str());
    }

    mvprintw(3, 1, format!("True floating point value: {}", float).as_str());
}

const MAX_POSITION: i32 = 32;
const MIN_POSITION: i32 = 1;

fn handle_input(c: i32, position: i32, current_bits: u32) -> (i32, u32) {
    match (c as u8) as char {
        'd' => (min(MAX_POSITION, position + 1), current_bits),
        'a' => (max(MIN_POSITION, position - 1), current_bits),
        'w' => {
            let bit = position - 1;

            (position, current_bits | (1 << (31 - bit)))
        },
        's' => {
            let bit = position - 1;

            (position, current_bits & !(1 << (31 - bit)))
        },
        _ => (position, current_bits),
    }
}

fn main() {
    let mut bits = (42.42f32).to_bits();
    let mut position: i32 = 0;

    initscr();
    start_color();

    init_pair(POINTER as i16, COLOR_BLACK, COLOR_WHITE);
    print_bit_patter(bits, position);
        
    loop {
        let c = getch();
        let (new_position, new_bits) = handle_input(c, position, bits);
        
        position = new_position;
        bits = new_bits;
        
        clear();
        print_bit_patter(bits, position);
        
        if c == 'q' as i32 {
            break;
        }

        refresh();
    }
    
    
    endwin();
}
 