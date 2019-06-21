use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    let mut positions: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("positions: {:?}", &positions);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    write_board(&mut handle)?;
    Ok(())
}

fn write_board(handle: &mut io::BufWriter<io::StdoutLock<>>) -> Result<(), io::Error> {
    writeln!(handle)?;
    writeln!(handle, "{} | {} | {}", "2", "o", "x")?;
    writeln!(handle, "{} {} {} {} {}", "-", "+", "-", "+", "-")?;
    writeln!(handle, "{} | {} | {}", "x", "o", "x")?;
    writeln!(handle, "{} {} {} {} {}", "-", "+", "-", "+", "-")?;
    writeln!(handle, "{} | {} | {}", "x", "o", "8")?;
    writeln!(handle)?;

    Ok(())
}