#[allow(unused_imports)]
pub mod prelude {
    pub mod crates {
        pub use color_eyre;
        pub use tracing;
        pub use tracing_subscriber;
    }
    pub use super::{observe, observe_info, trace_crate};
    pub use color_eyre::{Report, Result};
    pub use crates::*;
    pub use tracing::{Instrument, Level, debug, error, event, info, instrument, trace, warn};
    pub type Error = Report;
}

pub use prelude::crates::*;
use prelude::*;
pub use prelude::{Error, Report, Result};

#[instrument]
/// This function is marked unsafe because it's very likely you do not want to
/// do this.
///
/// Analagous to `observe("trace")`
pub unsafe fn very_verbosely_trace_everything() -> Result<()> {
    observe("trace")
}

#[instrument]
/// Analagous to `observe("info")`
pub fn observe_info() -> Result<()> {
    observe("info")
}

#[instrument]
/// Analagous to `observe(&format!("{}=trace,info", crate_name))`
pub fn trace_crate(crate_name: &str) -> Result<()> {
    observe(&format!("{}=trace,info", crate_name))
}

/// Sets up `color_eyre` and a basic logging tracing subscriber with the
/// specified `filter_str`. Everything a growing application needs.
#[instrument]
pub fn observe(filter_str: &str) -> Result<()> {
    color_eyre::install()?;

    // I'm guessing a v2 of this lib will involve a builder or registry so that
    // we can inject other subscribers for things like metrics exporters or file
    // loggers as well.
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(<tracing_subscriber::filter::Directive>::from(
            tracing::level_filters::LevelFilter::TRACE,
        ))
        .parse_lossy(filter_str);

    tracing_subscriber::fmt()
        // .pretty()
        .with_env_filter(filter)
        .init();

    debug!("setup complete");

    Ok(())
}
