use lyte_observer::prelude::*;

fn main() -> Result<()> {
    observe_info()?;
    info!("Hello, world!");
    debug!("This won't show up, though.");
    Ok(())
}
