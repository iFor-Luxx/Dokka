use windows::Win32::Graphics::Direct2D::Common::{
    D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_CLOSED,
};
use windows::Win32::Graphics::Direct2D::{ID2D1DeviceContext, ID2D1SolidColorBrush};
use windows::core::Result;
use windows_numerics::Vector2;

/// Rellena un triángulo con bordes suaves.
pub(crate) fn fill_triangle(
    context: &ID2D1DeviceContext,
    points: &[(f32, f32); 3],
    brush: &ID2D1SolidColorBrush,
) -> Result<()> {
    let [first, second, third] = points.map(|(x, y)| Vector2 { X: x, Y: y });
    // SAFETY: el contexto está en pleno dibujo; la figura se cierra antes de usarla.
    unsafe {
        let geometry = context.GetFactory()?.CreatePathGeometry()?;
        let sink = geometry.Open()?;
        sink.BeginFigure(first, D2D1_FIGURE_BEGIN_FILLED);
        sink.AddLine(second);
        sink.AddLine(third);
        sink.EndFigure(D2D1_FIGURE_END_CLOSED);
        sink.Close()?;
        context.FillGeometry(&geometry, brush, None);
    }
    Ok(())
}
