use super::status::{Settings, Status};
use super::numbers::NUMBERS;

use crossterm::{
    execute,
    terminal::size,
    style::{Print, Stylize, SetBackgroundColor, Color},
    cursor::MoveTo
};

use std::io::Stdout;

fn number_digit<T: ToString>(number: T, digit: usize) -> char {
    number.to_string().chars().nth(digit).unwrap_or(' ')

}

fn block_number_string(n: u8, x: u16, y: u16) -> Option<char> {
    assert!(n < 100);
    let first_digit = (n as f32 / 10.0).trunc() as usize;
    let second_digit = ((n as f32 / 10.0).fract() * 10.0) as usize;
    
    if y > 5 {
        return None
    }

    Some(match x {
        0..=6 => NUMBERS[first_digit].chars().nth( (x + (y*7)) as usize ).unwrap(),
        7 => ' ',
        8..=14 => NUMBERS[second_digit].chars().nth( ((x-8) + (y*7)) as usize ).unwrap(),
        _ => return None
    })
}

trait TerminalSection {
    // return Some(char) for an x coordinate, return None when end of line
    // NOTE: x may be greater than size as some x coordinates could be ANSI escape characters, which dont contribute to size
    fn output_at(x: u16, y: u16, settings: &Settings, status: &Status) -> Option<char>;
    // overall size of the section
    fn size() -> (u16, u16);
}

struct MinutesLeft {}
impl TerminalSection for MinutesLeft {
    fn output_at(x: u16, y: u16, settings: &Settings, status: &Status) -> Option<char> {
        let minutes_left = status.remaining_time_in_state(settings).minutes();

        if y == 5 {
            match x {
                0..=14 => return Some(' '),
                _ => return None
            }
        }

        if y == 6 {
            match x {
                0 => return Some(' '),
                1 => return Some('m'),
                2 => return Some('i'),
                3 => return Some('n'),
                4 => return Some('u'),
                5 => return Some('t'),
                6 => return Some('e'),
                7 => return Some('s'),
                8 => return Some(' '),
                9 => return Some('l'),
                10 => return Some('e'),
                11 => return Some('f'),
                12 => return Some('t'),
                13 => return Some(' '),
                14 => return Some(' '),
                _ => return None
            }
        }

        block_number_string(minutes_left as u8, x, y)
    }
    fn size() -> (u16, u16) {
        (15, 7)
    }
}

struct Completed {}
impl TerminalSection for Completed {
    fn output_at(x: u16, y: u16, settings: &Settings, status: &Status) -> Option<char> {
        Some(match (x, y) {
            (6, 3) => {
                number_digit(status.completed_pomodoros, 0)
            }
            (7, 3) => {
                number_digit(status.completed_pomodoros, 1)
            }
            (6, 4) => {
                number_digit(status.completed_work_time(settings).minutes(), 0)
            }
            (7, 4) => {
                number_digit(status.completed_work_time(settings).minutes(), 1)
            }
            (8, 4) => {
                number_digit(status.completed_work_time(settings).minutes(), 2)
            },
            _ => {
                [
                    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒".to_string(),
                    "▒▒                   ▒▒".to_string(),
            format!("▒▒     {}    ▒▒", r"Completed:".green().to_string()),
                    "▒▒    nn pomodoros   ▒▒".to_string(),
                    "▒▒    nnn minutes    ▒▒".to_string(),
                    "▒▒                   ▒▒".to_string(),
                    "▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒".to_string(),
                ].iter().nth(y as usize)?.chars().nth(x as usize)?
            }
        })

        
    }
    fn size() -> (u16, u16) {
        (23, 7)
    }
}

struct Pause {}
impl TerminalSection for Pause {
    fn output_at(x: u16, y: u16, _settings: &Settings, status: &Status) -> Option<char> {
        let bg_char = match status.hover_on_pause {
            true => "▓",
            false => "░"
        };
        let line = bg_char.repeat(13);
        Some(match status.paused {
            true => {
                let len_4_chars = bg_char.repeat(4);
                [
                    line.clone(),
                    line.clone(),
                    line.clone(),
            format!("{}{}{}", len_4_chars, "START".green().to_string(), len_4_chars),
                    line.clone(),
                    line.clone(),
                    line.clone(),
                ].iter().nth(y as usize)?.chars().nth(x as usize)?
            }
            false => {
                let len_3_chars = bg_char.repeat(3);
                let pause_line = format!("{}{}{}{}{}", len_3_chars, "██".red().to_string(), len_3_chars, "██".red().to_string(), len_3_chars);
                [
                    line.clone(),
                    line.clone(),
                    pause_line.clone(),
                    pause_line.clone(),
                    pause_line.clone(),
                    line.clone(),
                    line.clone(),
                ].iter().nth(y as usize)?.chars().nth(x as usize)?
            }
        })
    }
    fn size() -> (u16, u16) {
        (13, 7)
    }
}

struct Progress {}
impl TerminalSection for Progress {
    fn output_at(x: u16, y: u16, settings: &Settings, status: &Status) -> Option<char> {
        // handle max width of 100 borders
        if let 101.. = x {
            return None
        }

        match (x, y) {
            (0, 0) => return Some('┏'),
            (100, 0) => return Some('┓'),
            (0, 4) => return Some('┗'),
            (100, 4) => return Some('┛'),
            (0, _) => return Some('┃'),
            (100, _) => return Some('┃'),
            (_, 0) => return Some('━'),
            (_, 4) => return Some('━'),
            _ => ()
        };

        if status.in_break {
            match (x, y) {
                (48, 2) => return Some('B'),
                (49, 2) => return Some('r'),
                (50, 2) => return Some('e'),
                (51, 2) => return Some('a'),
                (52, 2) => return Some('k'),
                _ => ()
            }
        } else {
            match (x, y) {
                (48, 2) => return Some('W'),
                (49, 2) => return Some('o'),
                (50, 2) => return Some('r'),
                (51, 2) => return Some('k'),
                _ => ()
            }
        }

        let percent = status.fraction_of_state(settings);
        Some({
            let filled = (99.0*percent) as u16;
            if x == filled {
                '▒'
            }
            else if x + 1 == filled {
                '▓'
            }
            else if x <= filled {
                '█'
            } else {
                ' '
            }
        })
    }
    fn size() -> (u16, u16) {
        (101, 5)
    }
}

macro_rules! construct_section {
    ($section: tt, $term_size: ident, $buffer: ident, $settings: ident, $status: ident) => {
        let section_size = $section::size();
        let padding = " ".repeat((($term_size.0 - section_size.0) / 2) as usize);

        for y in 0..section_size.1 {
            $buffer.push_str(&padding);

            // dont use section size for x coordinate
            // the x coordinate is in terminal space, however output_at may return ANSI escape characters
            // for styling which don't contribute to terminal space. Iterator over x coordinates until output_at returns None.
            let mut x: u16 = 0;
            loop {
                let next_char = $section::output_at(x, y, $settings, $status);
                if next_char.is_none() { break }
                $buffer.push(next_char.unwrap());
                x += 1;
            }
            $buffer.push_str(&padding);
            $buffer.push('\n');
        }

    };
}

fn construct_string(settings: &Settings, status: &Status) -> String {
    let term_size = size().expect("Unable to get size of terminal.");
    let mut empty_line_full = " ".repeat(term_size.0 as usize);
    empty_line_full.push('\n');

    let mut inner_section_buffer = String::new();

    construct_section!(MinutesLeft, term_size, inner_section_buffer, settings, status);
    inner_section_buffer.push_str(&empty_line_full);
    inner_section_buffer.push_str(&empty_line_full);
    construct_section!(Completed, term_size, inner_section_buffer, settings, status);
    inner_section_buffer.push_str(&empty_line_full);
    inner_section_buffer.push_str(&empty_line_full);
    construct_section!(Pause, term_size, inner_section_buffer, settings, status);
    inner_section_buffer.push_str(&empty_line_full);
    inner_section_buffer.push_str(&empty_line_full);
    construct_section!(Progress, term_size, inner_section_buffer, settings, status);

    let lines = inner_section_buffer.split('\n');
    let inner_height = lines.count();

    let ih_u16 = inner_height as u16;
    let vertical_padding = if term_size.1 > ih_u16 { 
        (term_size.1 - ih_u16) / 2 
    } else {
        0
    };
    
    let mut buffer = String::with_capacity(((term_size.0+1)*term_size.1) as usize);

    for _y in 0..vertical_padding {
        buffer.push_str(&empty_line_full);
    }

    buffer.push_str(&inner_section_buffer);

    for _y in 0..vertical_padding {
        buffer.push_str(&empty_line_full);
    }

    buffer
}

pub fn output(mut stdout: &Stdout, settings: &Settings, status: &Status) {
    let string = construct_string(settings, status);
    let bg_cmd = match status.in_break {
        false => SetBackgroundColor(Color::Rgb { r: 20, g: 20, b: 30 }),
        true => SetBackgroundColor(Color::Rgb { r: 20, g: 80, b: 20 }),
    };
    let _ = execute!(stdout, bg_cmd, MoveTo(0, 0), Print(string));
}
