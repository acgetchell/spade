// Copyright 2016 The RTree Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use utils::{Vertex};
use spade::{RTree, DelaunayTriangulation, TrivialKernel};
use cgmath::{Vector2};
use glium::{DisplayBuild, Surface, VertexBuffer, Program, Display, DrawParameters};
use glium::glutin::{Event, ElementState};
use glium;

const VERTEX_SHADER_SRC: &'static str = r#"
    #version 140
    in vec2 pos;
    in vec3 color;

    out vec3 fragment_color;
    void main() {
    gl_Position = vec4(pos, 0.0, 1.0);
    fragment_color = color;
        }
    "#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
    #version 140
    out vec4 out_color;
    in vec3 fragment_color;
    void main() {
    out_color = vec4(fragment_color, 1.0);
        }
    "#;


pub struct ExampleApplication {
    pub tree: RTree<Vector2<f32>, Vector2<f32>>,
    pub delaunay: DelaunayTriangulation<Vector2<f32>, Vector2<f32>, TrivialKernel>,
    program: Program,
    pub edges_buffer: VertexBuffer<Vertex>,
    pub vertices_buffer: VertexBuffer<Vertex>,
    pub selection_buffer: VertexBuffer<Vertex>,
    pub display: Display,
}

impl ExampleApplication {
    pub fn new() -> ExampleApplication {
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(800, 800)
            .with_title("RTree Demo".to_string())
            .build_glium()
            .unwrap();
        let program = Program::from_source(&display, VERTEX_SHADER_SRC,
                                           FRAGMENT_SHADER_SRC, None).unwrap();
        let tree_edges_buffer = VertexBuffer::new(&display, &[]).unwrap();
        let tree_vertices_buffer = VertexBuffer::new(&display, &[]).unwrap();
        let selection_buffer = VertexBuffer::new(&display, &[]).unwrap();
        ExampleApplication {
            tree: RTree::new(),
            delaunay: Default::default(),
            display: display,
            program: program,
            edges_buffer: tree_edges_buffer,
            vertices_buffer: tree_vertices_buffer,
            selection_buffer: selection_buffer,
        }
    }
    
    pub fn default_handle_event(&mut self, event: &Event) -> bool {
        use glium::glutin::VirtualKeyCode::Escape;
        match event {
            &Event::Refresh => self.draw(),
            &Event::Closed | &Event::KeyboardInput(ElementState::Pressed, _, Some(Escape))
                => return true,
            _ => ()
        }
        false
    }

    pub fn draw(&self) {
        let mut target = self.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
        let parameters = DrawParameters {
            line_width: Some(1.0),
            .. Default::default()
        };


        target.draw(&self.edges_buffer, &indices, &self.program, 
                    &glium::uniforms::EmptyUniforms, &parameters).unwrap();

        let parameters = DrawParameters {
            point_size: Some(3.0),
            line_width: Some(2.0),
            .. Default::default()
        };

        target.draw(&self.selection_buffer, &indices, &self.program,
                    &glium::uniforms::EmptyUniforms, &parameters).unwrap();

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
        target.draw(&self.vertices_buffer, &indices, &self.program,
                    &glium::uniforms::EmptyUniforms, &parameters).unwrap();

        target.finish().unwrap();
    }
}
