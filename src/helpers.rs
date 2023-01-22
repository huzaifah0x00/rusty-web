use glium;
use glium::Frame;
use glium::Program;

use glium::Surface;
use glium::VertexBuffer;

#[derive(Copy, Clone)]
pub(crate) struct Vertex {
    pub position: [f32; 2],
}
glium::implement_vertex!(Vertex, position);

fn get_shader_program(display: &glium::Display) -> Program {
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    return Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
}

pub(crate) fn draw_triangle(vertices: Vec<Vertex>, display: &glium::Display, target: &mut Frame) {
    let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = get_shader_program(display);

    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
}
