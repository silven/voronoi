extern crate glium;

#[derive(Copy, Clone)]
pub struct PosOnlyVertex {
    pub position: [f32; 3],
}

implement_vertex!(PosOnlyVertex, position);

