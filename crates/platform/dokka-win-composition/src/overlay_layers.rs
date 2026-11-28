use dokka_scene::{Overlay, Panel, Shadow};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;

use crate::cursor_layer::CursorLayer;
use crate::divider_layer::DividerLayer;
use crate::dot_layer::DotLayer;
use crate::icon_layer::IconLayer;
use crate::overlay_piece_groups::PieceGroups;
use crate::overlay_style::{apply_scale, panel_layer};
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;
use crate::pointer_layer::{PointerLayer, pointer_style_of};
use crate::shadow_layer::ShadowLayer;
use crate::shadow_layer::others_of;
use crate::text_layer::TextLayer;

/// Elementos visuales de un panel flotante.
/// Orden: panel, píldora, punta, resaltado, líneas, puntos, textos, iconos.
/// Todo cuelga de un contenedor para fundir el panel sin tocar el dock ni las etiquetas.
/// El fondo puede no tener cristal: cuando el tinte y el borde del `Overlay.panel`
/// son transparentes (alfa 0, como el centro de control y el buscador) no se crea
/// ninguna capa para no ver un rectángulo borroso; el resto de paneles no cambia.
pub(crate) struct OverlayLayers {
    pub(crate) container: ContainerVisual,
    pieces: PieceGroups,
    pub(crate) panel: Option<PanelLayer>,
    pub(crate) panel_shadow: Option<ShadowLayer>,
    pub(crate) panel_shadow_spec: Option<Shadow>,
    pub(crate) tiles: Vec<PanelLayer>,
    /// Sombras bajo cada pieza (`None` si no se pidió o falló).
    pub(crate) tile_shadows: Vec<Option<ShadowLayer>>,
    /// Estilo con el que se creó cada pieza.
    pub(crate) tile_styles: Vec<Panel>,
    /// Sombra pedida bajo las piezas, si hay.
    pub(crate) tile_shadow: Option<Shadow>,
    /// Apariencia con la que se creó el fondo.
    pub(crate) style: Panel,
    pub(crate) pill: Option<PanelLayer>,
    /// Panel con el que se creó la píldora, si la hay.
    pub(crate) pill_style: Option<Panel>,
    pub(crate) pointer: Option<PointerLayer>,
    /// Altura y lado con los que se creó la punta, si la hay.
    pub(crate) pointer_style: Option<(f32, dokka_scene::PointerSide)>,
    pub(crate) highlight: Option<PanelLayer>,
    /// Estilo con el que se creó el resaltado, si lo hay.
    pub(crate) highlight_style: Option<Panel>,
    pub(crate) flats: Vec<PanelLayer>,
    /// Estilo con el que se creó cada plano.
    pub(crate) flat_styles: Vec<Panel>,
    pub(crate) dividers: Vec<DividerLayer>,
    pub(crate) cursors: Vec<CursorLayer>,
    pub(crate) dots: Vec<DotLayer>,
    pub(crate) texts: Vec<TextLayer>,
    pub(crate) icons: Vec<IconLayer>,
}

impl OverlayLayers {
    pub(crate) fn add(
        compositor: &Compositor,
        painter: &Painter,
        parent: &ContainerVisual,
        overlay: &Overlay,
    ) -> Result<Self> {
        let container = compositor.CreateContainerVisual()?;
        parent.Children()?.InsertAtTop(&container)?;
        container.SetOpacity(overlay.opacity)?;
        apply_scale(&container, overlay)?;
        let panel = panel_layer(compositor, &container, &overlay.panel)?;
        // La sombra del fondo excluye las piezas; las de cada pieza excluyen a
        // las demás para no oscurecerlas. Si fallan, se ve sin ellas.
        let panel_shadow_spec = overlay.panel_shadow;
        let panel_shadow = panel_shadow_spec.and_then(|shadow| {
            ShadowLayer::add(
                compositor,
                &container,
                &overlay.panel,
                &shadow,
                &overlay.tiles,
            )
            .ok()
        });
        // Cada pieza lleva lo suyo en su mismo contenedor para volar entera.
        let (pieces, map) = PieceGroups::build(compositor, &container, overlay)?;
        // Las piezas llevan filo de luz y brillo alto (Liquid Glass).
        let tiles = overlay
            .tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| {
                PanelLayer::add_tile(compositor, pieces.parent(&container, index), tile)
            })
            .collect::<Result<Vec<_>>>()?;
        let tile_shadows = overlay
            .tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| {
                overlay.tile_shadow.and_then(|shadow| {
                    ShadowLayer::add(
                        compositor,
                        pieces.parent(&container, index),
                        tile,
                        &shadow,
                        &others_of(&overlay.tiles, index),
                    )
                    .ok()
                })
            })
            .collect();
        // Encima del panel: el fondo de la píldora tapa ese tramo de tinte.
        let mut pill = None;
        if let Some(pill_panel) = &overlay.pill {
            pill = Some(PanelLayer::add(compositor, &container, pill_panel)?);
        }
        // Encima de la píldora para que su cristal tape ese tramo de borde y de tinte.
        let mut pointer = None;
        if let Some(style) = &overlay.pointer {
            pointer = Some(PointerLayer::add(
                compositor,
                &container,
                &overlay.panel,
                style,
            )?);
        }
        let mut highlight = None;
        if let Some(highlight_style) = &overlay.highlight {
            // Plano, sin cristal: el tinte de la píldora es translúcido y el
            // cristal muestrearía el fondo y lo taparía. El tinte del menú
            // es opaco y se ve igual sin cristal.
            highlight = Some(PanelLayer::add_flat(
                compositor,
                &container,
                highlight_style,
            )?);
        }
        let flats = overlay
            .flat_panels
            .iter()
            .enumerate()
            .map(|(index, panel)| {
                PanelLayer::add_flat(
                    compositor,
                    pieces.flat_parent(&container, &map, index),
                    panel,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let dividers = overlay
            .dividers
            .iter()
            .enumerate()
            .map(|(index, divider)| {
                DividerLayer::add(
                    compositor,
                    pieces.divider_parent(&container, &map, index),
                    divider,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let cursors = overlay
            .cursors
            .iter()
            .enumerate()
            .map(|(index, cursor)| {
                CursorLayer::add(
                    compositor,
                    pieces.cursor_parent(&container, &map, index),
                    cursor,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let dots = overlay
            .dots
            .iter()
            .enumerate()
            .map(|(index, dot)| {
                DotLayer::add(compositor, pieces.dot_parent(&container, &map, index), dot)
            })
            .collect::<Result<Vec<_>>>()?;
        let texts = overlay
            .texts
            .iter()
            .enumerate()
            .map(|(index, _)| {
                TextLayer::add(compositor, pieces.text_parent(&container, &map, index))
            })
            .collect::<Result<Vec<_>>>()?;
        // Encima del resaltado y junto a los textos: no se solapan con ellos.
        let icons = overlay
            .icons
            .iter()
            .enumerate()
            .map(|(index, icon)| {
                IconLayer::add(
                    compositor,
                    painter,
                    pieces.icon_parent(&container, &map, index),
                    icon,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            container,
            pieces,
            panel,
            panel_shadow,
            panel_shadow_spec,
            tiles,
            tile_shadows,
            tile_styles: overlay.tiles.clone(),
            tile_shadow: overlay.tile_shadow,
            style: overlay.panel,
            pill,
            pill_style: overlay.pill,
            pointer,
            pointer_style: overlay.pointer.map(pointer_style_of),
            highlight,
            highlight_style: overlay.highlight,
            flats,
            flat_styles: overlay.flat_panels.clone(),
            dividers,
            cursors,
            dots,
            texts,
            icons,
        })
    }

    /// Lanza la salida por piezas con la orden de la isla. Falla si las
    /// piezas ya no coinciden: el shell cierra sin animación.
    pub(crate) fn start_dismiss(
        &self,
        compositor: &Compositor,
        dismiss: &dokka_scene::ControlDismiss,
    ) -> Result<()> {
        self.pieces.start(compositor, &self.tile_styles, dismiss)
    }
}
