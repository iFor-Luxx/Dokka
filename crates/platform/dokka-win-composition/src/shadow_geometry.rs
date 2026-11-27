//! Recorte vectorial de la sombra: el exterior menos el panel y sus vecinos.

use dokka_scene::Panel;
use windows::Graphics::{IGeometrySource2D, IGeometrySource2D_Impl};
use windows::UI::Composition::{CompositionPath, CompositionPathGeometry, Compositor};
use windows::Win32::Graphics::Direct2D::Common::{D2D_RECT_F, D2D1_FILL_MODE_ALTERNATE};
use windows::Win32::Graphics::Direct2D::{
    D2D1_COMBINE_MODE_EXCLUDE, D2D1_ROUNDED_RECT, ID2D1Factory, ID2D1Geometry,
};
use windows::Win32::System::WinRT::Graphics::Direct2D::{
    IGeometrySource2DInterop, IGeometrySource2DInterop_Impl,
};
use windows::core::{Interface, Ref, Result, implement};

/// Fuente de geometría 2D que envuelve la D2D combinada para `CompositionPath`.
#[implement(IGeometrySource2D, IGeometrySource2DInterop)]
struct GeometrySource {
    geometry: ID2D1Geometry,
}

impl IGeometrySource2D_Impl for GeometrySource_Impl {}

impl IGeometrySource2DInterop_Impl for GeometrySource_Impl {
    fn GetGeometry(&self) -> Result<ID2D1Geometry> {
        Ok(self.geometry.clone())
    }

    fn TryGetGeometryUsingFactory(&self, _factory: Ref<ID2D1Factory>) -> Result<ID2D1Geometry> {
        Ok(self.geometry.clone())
    }
}

/// Anillo exterior menos interior (y vecinos): rectángulo fuera, redondeados
/// dentro, en coordenadas del propio visual. Todo vectorial, sin imágenes.
pub(crate) fn build_path(
    compositor: &Compositor,
    factory: &ID2D1Factory,
    panel: &Panel,
    occluders: &[Panel],
    margin: f32,
) -> Result<CompositionPathGeometry> {
    let width = panel.bounds.width;
    let height = panel.bounds.height;
    let radius = panel
        .corner_radius
        .min(width / 2.0)
        .min(height / 2.0)
        .max(0.0);
    // SAFETY: la factoría y las geometrías solo se usan en este hilo; el
    // `sink` se cierra siempre antes de crear el camino de composición.
    let d2d = unsafe {
        let outer = D2D_RECT_F {
            left: -margin,
            top: -margin,
            right: width + margin,
            bottom: height + margin,
        };
        let outer_geo = factory.CreateRectangleGeometry(&outer)?;
        let mut inners: Vec<Option<ID2D1Geometry>> = Vec::new();
        let inner = D2D1_ROUNDED_RECT {
            rect: D2D_RECT_F {
                left: 0.0,
                top: 0.0,
                right: width,
                bottom: height,
            },
            radiusX: radius,
            radiusY: radius,
        };
        let inner_geo: ID2D1Geometry = factory.CreateRoundedRectangleGeometry(&inner)?.cast()?;
        inners.push(Some(inner_geo));
        for occ in occluders {
            if occ.bounds.width <= 0.0 || occ.bounds.height <= 0.0 {
                continue;
            }
            let local_x = occ.bounds.x - panel.bounds.x;
            let local_y = occ.bounds.y - panel.bounds.y;
            let oradius = occ
                .corner_radius
                .min(occ.bounds.width / 2.0)
                .min(occ.bounds.height / 2.0)
                .max(0.0);
            let rounded = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: local_x,
                    top: local_y,
                    right: local_x + occ.bounds.width,
                    bottom: local_y + occ.bounds.height,
                },
                radiusX: oradius,
                radiusY: oradius,
            };
            let geo: ID2D1Geometry = factory.CreateRoundedRectangleGeometry(&rounded)?.cast()?;
            inners.push(Some(geo));
        }
        let group = factory.CreateGeometryGroup(D2D1_FILL_MODE_ALTERNATE, &inners)?;
        let path_geo = factory.CreatePathGeometry()?;
        let sink = path_geo.Open()?;
        outer_geo.CombineWithGeometry(&group, D2D1_COMBINE_MODE_EXCLUDE, None, 0.25, &sink)?;
        sink.Close()?;
        let combined: ID2D1Geometry = path_geo.cast()?;
        combined
    };
    let source: IGeometrySource2D = GeometrySource { geometry: d2d }.into();
    let path = CompositionPath::Create(&source)?;
    compositor.CreatePathGeometryWithPath(&path)
}
