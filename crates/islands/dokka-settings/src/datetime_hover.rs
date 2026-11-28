//! Resaltado de los controles nuevos de "Fecha y hora".
//!
//! Vive aparte para que `pointer.rs` no supere el límite por archivo.

use super::datetime_edit::visible_zones;
use super::general::GeneralPage;
use super::logic::{
    auto_time_at, auto_zone_at, hour24_at, manual_hit_at, server_hit_at, zone_hit_at,
};
use super::painted::PaintedControl;
use super::state::Settings;

/// Hover de los nuevos controles de fecha, si responde.
pub(crate) fn datetime_hover(settings: &Settings, x: f32, y: f32) -> Option<PaintedControl> {
    if settings.general() != GeneralPage::DateTime {
        return None;
    }
    let width = settings.content_width();
    let scroll = settings.scroll_offset();
    let edit = settings.datetime_edit();
    let summary = settings.system_summary().unwrap_or_default();
    let auto_on = summary.datetime.auto_time == Some(true);
    let auto_zone_on = summary.datetime.auto_timezone == Some(true);
    if hour24_at(edit, width, scroll, x, y) {
        return Some(PaintedControl::DateTimeSwitch);
    }
    if edit.can_change && auto_time_at(edit, width, scroll, x, y) {
        return Some(PaintedControl::DateTimeAutoTime);
    }
    if edit.can_change && auto_zone_at(edit, width, scroll, x, y) {
        return Some(PaintedControl::DateTimeAutoZone);
    }
    if edit.can_change
        && let Some(hit) = server_hit_at(edit, width, scroll, x, y)
    {
        use super::logic::ServerHit as Hit;
        match hit {
            Hit::Change => return Some(PaintedControl::DateTimeServer(0)),
            Hit::Apply => return Some(PaintedControl::DateTimeServer(1)),
            Hit::Cancel => return Some(PaintedControl::DateTimeServer(2)),
        }
    }
    if let Some(hit) = manual_hit_at(edit, auto_on, width, scroll, x, y) {
        use super::logic::ManualHit as Hit;
        if !edit.can_change {
            return None;
        }
        match hit {
            Hit::Change => return Some(PaintedControl::DateTimeManual(0)),
            Hit::Apply => return Some(PaintedControl::DateTimeManual(1)),
            Hit::Cancel => return Some(PaintedControl::DateTimeManual(2)),
            Hit::Step(field, delta) => return Some(PaintedControl::DateTimeStep(field, delta)),
        }
    }
    if let Some(hit) = zone_hit_at(edit, auto_zone_on, width, scroll, x, y) {
        use super::logic::ZoneHit as Hit;
        match hit {
            Hit::Toggle => return Some(PaintedControl::DateTimeZoneToggle),
            Hit::Select(slot) => {
                let slot = slot.min(visible_zones().saturating_sub(1)) as u8;
                return Some(PaintedControl::DateTimeZoneSlot(slot));
            }
        }
    }
    None
}
