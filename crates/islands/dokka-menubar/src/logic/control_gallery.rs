//! Galería del modo editar: categorías, búsqueda y vuelta al centro.
//!
//! La isla guarda la categoría, lo escrito y los dos desplazamientos; el
//! componente `control_gallery` solo recibe y dibuja. Un clic en un control
//! quitado lo añade al momento con el mismo `HiddenControlsChanged` (se
//! guarda por el mismo camino); lo que ya está en el centro sale atenuado
//! y no responde. "Aceptar" cierra la galería y el modo editar.

use dokka_components::control_gallery::{
    CATEGORY_COUNT, ControlGalleryProps, GalleryHit, clamp_grid_scroll, clamp_list_scroll,
    gallery_contains, gallery_hit, gallery_panel, grid_view, list_view,
};
use dokka_scene::{Overlay, Rect};
use dokka_theme::control_gallery as measures;

use crate::{Menubar, MenubarAction};

impl Menubar {
    /// `true` con la galería abierta: en modo editar con el centro abierto.
    pub fn is_gallery_open(&self) -> bool {
        self.is_control_editing()
    }

    /// Ventana de la galería para este monitor, sin tapar el centro.
    pub(crate) fn gallery_rect(&self, bar_width: f32, screen_height: f32) -> Option<Rect> {
        if !self.is_gallery_open() {
            return None;
        }
        Some(measures::gallery_rect(
            bar_width,
            screen_height,
            self.tray_bounds(),
        ))
    }

    /// Alto de ventana necesario con la galería: todo el alto de la
    /// pantalla, como el buscador, para que quepa centrada.
    pub fn gallery_window_height(&self, screen_height: f32) -> f32 {
        if screen_height.is_nan() || screen_height <= 0.0 {
            return dokka_theme::menubar::WINDOW_HEIGHT;
        }
        screen_height
    }

    fn gallery_props(&self) -> ControlGalleryProps {
        ControlGalleryProps {
            center: self.control_props(),
            selected: self.gallery_category.min(CATEGORY_COUNT - 1),
            query: self.gallery_query.clone(),
        }
    }

    pub(crate) fn gallery_overlay(
        &self,
        bar_width: f32,
        screen_height: f32,
    ) -> Option<Overlay> {
        let gallery = self.gallery_rect(bar_width, screen_height)?;
        Some(gallery_panel(
            &self.theme,
            &self.control_icons,
            self.search_icons.field(),
            &self.gallery_badges,
            gallery,
            &self.gallery_props(),
            self.gallery_list_scroll,
            self.gallery_grid_scroll,
            self.gallery_cursor,
        ))
    }

    /// `true` si el punto cae en la galería abierta.
    pub(crate) fn gallery_at(&self, x: f32, y: f32, bar_width: f32, screen_height: f32) -> bool {
        let Some(gallery) = self.gallery_rect(bar_width, screen_height) else {
            return false;
        };
        gallery_contains(gallery, x, y)
    }

    /// Letra escrita: filtra la galería y parpadea su cursor.
    pub fn gallery_typed(&mut self, character: char, bar_width: f32, screen_height: f32) {
        if !self.is_gallery_open() {
            return;
        }
        self.gallery_query.push(character);
        self.gallery_cursor = true;
        self.clamp_gallery_scroll(bar_width, screen_height);
    }

    /// Retroceso: borra la última letra de la galería.
    pub fn gallery_erased(&mut self, bar_width: f32, screen_height: f32) {
        if !self.is_gallery_open() {
            return;
        }
        self.gallery_query.pop();
        self.gallery_cursor = true;
        self.clamp_gallery_scroll(bar_width, screen_height);
    }

    /// Parpadeo del cursor de la galería, una vez por `Tick`.
    pub fn tick_gallery_cursor(&mut self) {
        if self.is_gallery_open() {
            self.gallery_cursor = !self.gallery_cursor;
        }
    }

    /// Rueda sobre la galería: desplaza su columna o sus secciones con la
    /// posición del cursor, sin que nada se salga del panel.
    pub fn gallery_scroll(
        &mut self,
        x: f32,
        y: f32,
        delta: i16,
        bar_width: f32,
        screen_height: f32,
    ) {
        if !self.is_gallery_open() {
            return;
        }
        let Some(gallery) = self.gallery_rect(bar_width, screen_height) else {
            return;
        };
        let step = -(delta as f32) / 120.0 * measures::SCROLL_STEP;
        if contains(list_view(gallery), x, y) {
            self.gallery_list_scroll =
                clamp_list_scroll(gallery, self.gallery_list_scroll + step);
        } else if contains(grid_view(gallery), x, y) {
            let props = self.gallery_props();
            self.gallery_grid_scroll = clamp_grid_scroll(
                gallery,
                props.selected,
                &props.query,
                self.brightness.is_some(),
                self.gallery_grid_scroll + step,
            );
        }
    }

    fn clamp_gallery_scroll(&mut self, bar_width: f32, screen_height: f32) {
        let Some(gallery) = self.gallery_rect(bar_width, screen_height) else {
            return;
        };
        let props = self.gallery_props();
        self.gallery_list_scroll = clamp_list_scroll(gallery, self.gallery_list_scroll);
        self.gallery_grid_scroll = clamp_grid_scroll(
            gallery,
            props.selected,
            &props.query,
            self.brightness.is_some(),
            self.gallery_grid_scroll,
        );
    }

    /// Clic en la galería: elige categoría, añade el control quitado al
    /// momento o acepta y cierra. Lo demás (casilla y atenuados) no hace
    /// nada y no cierra.
    pub(crate) fn gallery_click(
        &mut self,
        x: f32,
        y: f32,
        bar_width: f32,
        screen_height: f32,
    ) -> Option<MenubarAction> {
        let gallery = self.gallery_rect(bar_width, screen_height)?;
        let props = self.gallery_props();
        match gallery_hit(
            gallery,
            props.selected,
            &props.query,
            self.brightness.is_some(),
            self.gallery_list_scroll,
            self.gallery_grid_scroll,
            x,
            y,
        )? {
            GalleryHit::Category(index) => {
                self.gallery_category = index.min(CATEGORY_COUNT - 1);
                self.gallery_grid_scroll = 0.0;
                None
            }
            GalleryHit::Control(item) => {
                if !self.shown.hidden_controls.contains(item) {
                    return None;
                }
                let mut hidden = self.shown.hidden_controls;
                hidden.remove(item);
                self.shown.hidden_controls = hidden;
                self.resize_control_center();
                self.clamp_gallery_scroll(bar_width, screen_height);
                Some(MenubarAction::HiddenControlsChanged(hidden))
            }
            GalleryHit::Search => None,
            GalleryHit::Accept => {
                self.close_tray();
                None
            }
        }
    }
}

fn contains(area: Rect, x: f32, y: f32) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
