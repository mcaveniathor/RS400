use rs400::{System};

fn main() -> anyhow::Result<()> {
    let system = System::open()?;

    // Flush database to disk before exiting.
    system.flush()?;
    Ok(())
}
