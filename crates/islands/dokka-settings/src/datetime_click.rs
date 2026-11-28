//! Clic en los controles nuevos de "Fecha y hora".
//!
//! Vive aparte para que `controls.rs` no supere el límite por archivo.
//! La isla solo emite acciones; el shell conecta y pide releer.

use super::general::GeneralPage;
use super::logic::{
    ManualHit, ServerHit, ZoneHit, auto_time_at, auto_zone_at, hour24_at, manual_hit_at,
    server_hit_at, zone_hit_at,
};
use super::state::Settings;
use crate::SettingsAction;

/// Acción de fecha de un clic, si cae en sus controles nuevos.
pub(crate) fn datetime_edit_action(settings: &mut Settings, x: f32, y: f32) -> Option<SettingsAction> {
    if settings.general() != GeneralPage::DateTime {
        return None;
    }
    let width = settings.content_width();
    let scroll = settings.scroll_offset();
    if hour24_at(&settings.datetime_edit, width, scroll, x, y) {
        let mut preferences = settings.values().preferences;
        preferences.general.hour_24 = !preferences.general.hour_24;
        settings.values_mut().preferences = preferences;
        return Some(SettingsAction::Changed(preferences));
    }
    let summary = settings.system_summary().unwrap_or_default();
    let auto_on = summary.datetime.auto_time == Some(true);
    let auto_zone_on = summary.datetime.auto_timezone == Some(true);
    let can_change = settings.datetime_edit.can_change;
    if can_change && auto_time_at(&settings.datetime_edit, width, scroll, x, y) {
        let next = summary.datetime.auto_time != Some(true);
        return Some(SettingsAction::DateTimeAutoTime(next));
    }
    if can_change && auto_zone_at(&settings.datetime_edit, width, scroll, x, y) {
        let next = summary.datetime.auto_timezone != Some(true);
        return Some(SettingsAction::DateTimeAutoZone(next));
    }
    if can_change && let Some(hit) = server_hit_at(&settings.datetime_edit, width, scroll, x, y) {
        match hit {
            ServerHit::Change => {
                if !settings.datetime_edit.server_editing {
                    let current = summary.datetime.ntp_server.clone().unwrap_or_default();
                    settings.datetime_edit_mut().start_server(&current);
                }
                return Some(SettingsAction::Stay);
            }
            ServerHit::Apply => {
                let draft = settings.datetime_edit.server_draft.trim().to_owned();
                if !crate::datetime_edit::is_valid_server(&draft) {
                    return Some(SettingsAction::Stay);
                }
                settings.datetime_edit_mut().cancel_server();
                return Some(SettingsAction::DateTimeServer(draft));
            }
            ServerHit::Cancel => {
                settings.datetime_edit_mut().cancel_server();
                return Some(SettingsAction::Stay);
            }
        }
    }
    if let Some(hit) = manual_hit_at(&settings.datetime_edit, auto_on, width, scroll, x, y) {
        if !can_change {
            return None;
        }
        match hit {
            ManualHit::Change => {
                if !settings.datetime_edit.manual_editing
                    && let Some(now) = settings.page_now()
                {
                    settings.datetime_edit_mut().start_manual(now);
                }
                return Some(SettingsAction::Stay);
            }
            ManualHit::Apply => {
                let edit = settings.datetime_edit.clone();
                settings.datetime_edit_mut().cancel_manual();
                return Some(SettingsAction::DateTimeManual(
                    crate::datetime_edit::DateTimeEditManual {
                        year: edit.year,
                        month: edit.month,
                        day: edit.day,
                        hour: edit.hour,
                        minute: edit.minute,
                    },
                ));
            }
            ManualHit::Cancel => {
                settings.datetime_edit_mut().cancel_manual();
                return Some(SettingsAction::Stay);
            }
            ManualHit::Step(field, delta) => {
                settings.datetime_edit_mut().step(field, delta);
                return Some(SettingsAction::Stay);
            }
        }
    }
    if let Some(hit) = zone_hit_at(&settings.datetime_edit, auto_zone_on, width, scroll, x, y) {
        match hit {
            ZoneHit::Toggle => {
                settings.datetime_edit_mut().toggle_zones();
                return Some(SettingsAction::Stay);
            }
            ZoneHit::Select(index) => {
                let key = settings.datetime_edit.zones.get(index)?.key.clone();
                settings.datetime_edit_mut().zone_open = false;
                return Some(SettingsAction::DateTimeZone(key));
            }
        }
    }
    None
}
