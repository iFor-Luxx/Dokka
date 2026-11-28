use super::general::{GeneralPage, PageNow};
use super::logic::SliderTarget;
use super::values::ContentValues;

/// Lo que cambia lo que se ve del contenido: valores, hover y arrastre.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaintedControl {
    /// Interruptor del Dock (0 ampliación, 1 ocultado, 2 animar,
    /// 3 indicadores, 4 vistas previas).
    DockSwitch(usize),
    /// Interruptor de la barra superior (fila 0-6).
    MenubarSwitch(usize),
    /// Interruptor de la barra de tareas.
    TaskbarSwitch,
    /// Deslizador del Dock: 0 tamaño, 1 ampliación.
    DockSlider(u8),
    /// Fila de la lista de General (0-4): resalta al pasar el cursor.
    GeneralRow(usize),
    /// Botón de "Acerca de este PC": 0 pantalla, 1 almacenamiento, 2 informe.
    AboutButton(u8),
    /// Interruptor de "Fecha y hora": formato de 24 horas.
    DateTimeSwitch,
    /// Interruptor de automáticas de Windows.
    DateTimeAutoTime,
    /// Interruptor de zona automática.
    DateTimeAutoZone,
    /// Servidor: 0 cambiar, 1 aplicar, 2 cancelar.
    DateTimeServer(u8),
    /// Fecha manual: 0 cambiar, 1 aplicar, 2 cancelar.
    DateTimeManual(u8),
    /// Paso de fecha: campo 0-4 y sentido.
    DateTimeStep(u8, i8),
    /// Desplegable de zonas: cabecera.
    DateTimeZoneToggle,
    /// Zona de la lista (posición visible 0-5).
    DateTimeZoneSlot(u8),
    /// Segmentado de "Idioma y región": primer día de la semana.
    LanguageSegment,
    /// Botón de "Fecha y hora" (0) o de "Idioma y región" (1).
    GeneralButton(u8),
    /// Botón de "Almacenamiento": 0 vaciar papelera, 1 ajustes.
    StorageButton(u8),
    /// Fila de desinstalar en "Actualización de Dokka".
    Uninstall,
}

/// Todo lo que cambia lo que se ve: si no cambió, el shell no repinta.
#[derive(Debug, Clone, PartialEq)]
pub struct PaintedSettings {
    pub selected: usize,
    pub general: GeneralPage,
    pub hovered: Option<usize>,
    pub nav_hovered: Option<dokka_components::nav_pill::NavHalf>,
    pub traffic_hovered: bool,
    pub window_focused: bool,
    pub zoomed: bool,
    pub query: String,
    pub search_focused: bool,
    pub can_previous: bool,
    pub can_next: bool,
    /// Valores reales del contenido.
    pub preferences: dokka_model::Preferences,
    pub taskbar_auto: bool,
    /// Control bajo el cursor, si hay alguno.
    pub control: Option<PaintedControl>,
    /// Deslizador a medias: cuál (0 tamaño, 1 ampliación) y su valor de 0 a 1.
    pub drag: Option<(u8, OrderedF32)>,
    /// Fila de la barra a medias: de dónde a dónde y a qué altura flotante.
    pub reorder: Option<PaintedReorder>,
    /// Desplazamiento vertical del contenido de General.
    pub scroll: OrderedF32,
    /// Resumen del equipo para "Acerca de este PC".
    pub system: Option<dokka_model::SystemSummary>,
    /// Fecha y hora que muestra "Fecha y hora", en vivo con el `Tick`.
    pub now: Option<PageNow>,
    /// Desglose de almacenamiento para su apartado.
    pub storage: Option<dokka_model::StorageBreakdown>,
    /// Versión instalada de Dokka para su apartado.
    pub dokka_version: Option<String>,
    /// Si el botón de desinstalar responde (instalado).
    pub uninstall_available: bool,
    /// Si lo protegido de fecha responde (elevado).
    pub datetime_can_change: bool,
    /// Cuántas zonas trae el desplegable.
    pub datetime_zones: usize,
    /// Casilla del servidor abierta y su borrador.
    pub server_editing: bool,
    pub server_draft: String,
    /// Editor de fecha abierto y su borrador.
    pub manual_editing: bool,
    pub manual_day: u16,
    pub manual_month: u16,
    pub manual_year: u16,
    pub manual_hour: u16,
    pub manual_minute: u16,
    /// Desplegable de zonas abierto y su desplazamiento.
    pub zone_open: bool,
    pub zone_scroll: usize,
}

/// Fila arrastrada en la huella: hueco de origen, destino y altura.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaintedReorder {
    pub from: usize,
    pub target: usize,
    pub height: OrderedF32,
}

/// `f32` comparable para la huella de repintado: el arrastre nunca es `NaN`.
#[derive(Debug, Clone, Copy)]
pub struct OrderedF32(pub f32);

impl PartialEq for OrderedF32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

/// Deslizador a medias en la forma que guarda la huella.
pub(crate) fn painted_drag(values: &ContentValues) -> Option<(u8, OrderedF32)> {
    values.drag.map(|target| {
        let kind = match target {
            SliderTarget::DockScale => 0,
            SliderTarget::DockStrength => 1,
        };
        (kind, OrderedF32(values.drag_value))
    })
}

/// Fila de la barra a medias: solo si ya pasó el umbral y se mueve.
pub(crate) fn painted_reorder(values: &ContentValues) -> Option<PaintedReorder> {
    let drag = values.reorder?;
    if !drag.is_active() {
        return None;
    }
    Some(PaintedReorder {
        from: drag.from(),
        target: drag.target(),
        height: OrderedF32(drag.current_y()),
    })
}
