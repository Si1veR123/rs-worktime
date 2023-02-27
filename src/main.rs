mod output;
mod status;
mod long_duration;
mod numbers;

use status::{Settings, Status};
use long_duration::LongDuration;
use output::output;

use crossterm::{
    style::{SetBackgroundColor, Color},
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, size},
    execute,
    Result,
    cursor::{Hide, Show},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers, poll, MouseEvent, MouseEventKind, EnableMouseCapture}
};

use std::env::args;
use std::thread;
use std::time::Duration;
use std::io;

fn in_pause_box(col: u16, row: u16) -> bool {
    let term_size = size().unwrap();
    let upper_left_of_inner = ((term_size.0 / 2) - 50, (term_size.1 / 2) - 17);

    let x_range = (upper_left_of_inner.0 + 44)..=(upper_left_of_inner.0 + 57);
    let y_range = (upper_left_of_inner.1 + 18)..=(upper_left_of_inner.1 + 25);
    
    if x_range.contains(&col) & y_range.contains(&row) {
        return true
    }

    false
}

fn help() {
    println!("Usage:\n(exe) <work time mins> <short break time> <long break time> <long break cycles>")
}

fn main() -> Result<()> {
    let cmd_args = args().skip(1);
    let args_vec: Vec<String> = cmd_args.collect();

    let settings = if args_vec.is_empty() {
        Settings::default()
    } else if args_vec.len() == 4 {
        let mut times = args_vec.iter().map(|x| x.parse().expect("Argument wasn't an integer."));

        Settings {
            work_time:  LongDuration::new_minutes(times.next().unwrap()),
            short_break_time: LongDuration::new_minutes(times.next().unwrap()),
            long_break_time: LongDuration::new_minutes(times.next().unwrap()),
            long_break_cycles: times.next().unwrap() as usize,
        }
    } else {
        println!("Invalid number of arguments. Help: ");
        help();
        return Ok(());
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        SetBackgroundColor(
            Color::Rgb { r: 6, g: 6, b: 30 }
        ),
        Hide,
        EnableMouseCapture
    )?;

    let mut status = Status::initial();

    loop {
        output(&stdout, &settings, &status);
        thread::sleep(Duration::from_secs(1));
        status.update(&settings, &LongDuration::new_seconds(1));


        if poll(Duration::from_secs(0)).unwrap() {
            let ev = read().unwrap();
            match ev {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    // quit
                    break
                },

                Event::Mouse(
                    MouseEvent {
                        kind: MouseEventKind::Moved,
                        column,
                        row,
                        ..
                    }
                ) => {
                    // mouse drag
                    status.hover_on_pause = in_pause_box(column, row);
                },

                Event::Mouse(
                    MouseEvent {
                        kind: MouseEventKind::Down(_),
                        column,
                        row,
                        ..
                    }
                ) => {
                    // button click
                    if in_pause_box(column, row) {
                        match status.paused {
                            true => status.paused = false,
                            false => status.paused = true
                        }
                    }
                }

                _ => ()
            };
        }
    }

    execute!(
        stdout,
        LeaveAlternateScreen,
        Show
    )?;

    Ok(())
}
