//! Movimientos calculados a partir del tiempo transcurrido.

mod bounce;
mod ease;
mod genie;
mod spring;

pub use bounce::Bounce;
pub use ease::{ease_out_back, ease_out_cubic};
pub use genie::{
    Edges as GenieEdges, GENIE_AIM_FRAMES, GENIE_CAPTURE_TIMEOUT_SECONDS,
    GENIE_FLIGHT_TIMEOUT_SECONDS, GENIE_HANDOVER_FRAMES, GENIE_LANDING_START, GENIE_MAX_STRIPS,
    GENIE_OVERLAP_ROWS, GENIE_SECONDS, GENIE_SETTLE_SECONDS, GENIE_STRIP_ROWS, Genie,
    Strip as GenieStrip, flight_progress, landing_opacity, strip_count,
};
pub use spring::Spring;
