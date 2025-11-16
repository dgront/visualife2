use crate::PySvgDrawing;

pub trait DrawingComposit {
    fn add_composit_to_drawing(&self, drawing: &mut PySvgDrawing) -> PyResult<()>;
}
