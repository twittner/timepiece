use std::io;
use std::time::Duration;
use std::thread::sleep;

use anyhow::Result;
use crossterm::{QueueableCommand, cursor, terminal::{Clear, ClearType}};
use jiff::{Unit, Zoned, civil::TimeRound};

const FREQ: Duration = Duration::from_millis(1000);

fn main() -> Result<()> {
    let round = TimeRound::from((Unit::Millisecond, 100));

    let mut stdout = io::stdout();

    clear_all(&mut stdout)?;

    let now = Zoned::now().round(Unit::Second)?;
    println!("{:?} {} {}", now.weekday(), now.date(), now.time().round(round)?);

    loop {
        let now = Zoned::now().round(Unit::Millisecond)?.time();
        if now.millisecond() == 0 {
            break
        }
        sleep(Duration::from_millis(1))
    }

    clear_line(&mut stdout)?;

    loop {
        let now = Zoned::now().round(Unit::Millisecond)?;
        println!("{:?} {} {}", now.weekday(), now.date(), now.time().round(round)?);
        let delta = Duration::from_millis(now.time().millisecond() as u64);
        sleep(FREQ.saturating_sub(delta));
        clear_line(&mut stdout)?;
    }
}

fn clear_all<W: io::Write>(w: &mut W) -> io::Result<()> {
    w.queue(cursor::MoveTo(0, 0))?
     .queue(cursor::Hide)?
     .queue(Clear(ClearType::All))?
     .flush()
}

fn clear_line<W: io::Write>(w: &mut W) -> io::Result<()> {
    w.queue(cursor::MoveToPreviousLine(1))?
     .queue(Clear(ClearType::CurrentLine))?
     .flush()
}

