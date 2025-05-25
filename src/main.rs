use std::io::{self, Write};
use std::time::Duration;
use std::thread::sleep;

use anyhow::Result;
use crossterm::{QueueableCommand, cursor, terminal::{Clear, ClearType}};
use jiff::{Unit, Zoned};

const FREQ: Duration = Duration::from_millis(1000);

fn main() -> Result<()> {
    let mut stdout = io::stdout();

    stdout.queue(cursor::MoveTo(0, 0))?
        .queue(cursor::Hide)?
        .queue(Clear(ClearType::All))?
        .flush()?;

    loop {
        let now = Zoned::now().round(Unit::Millisecond)?;
        println!("{:?} {} {}", now.weekday(), now.date(), now.time().round(Unit::Second)?);
        let delta = Duration::from_millis(now.time().millisecond() as u64);
        sleep(FREQ.saturating_sub(delta));
        stdout.queue(cursor::MoveToPreviousLine(1))?
            .queue(Clear(ClearType::CurrentLine))?
            .flush()?
    }
}
