use std::io::{self, Write, BufRead};

fn main() -> io::Result<()> {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut handle = stdin.lock();
    let mut w = stdout.lock();
    let mut buf = String::new();
    let mut iter = vec.iter();
    write!(w, "> ")?;
    w.flush()?;
    while let Ok(len) = handle.read_line(&mut buf) {
        match &buf[..len-1] {
            "" => {},
            "n" | "next" => {
                writeln!(w, "Next: {:?}", iter.next())?;
            },
            "p" | "prev" => {
                writeln!(w, "Prev: {:?}", iter.next_back())?;
            },
            "q" | "quit" | "exit" => break,
            c => writeln!(w, "Err: No command for {}", c)?
        }
        w.flush()?;
        write!(w, "> ")?;
        w.flush()?;
        buf.clear();
    }
    Ok(())
}
