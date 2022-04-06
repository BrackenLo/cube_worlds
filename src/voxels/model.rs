//====================================================================

//====================================================================

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<Mesh>,
}
impl Model {
    pub fn move_model(&mut self, translation: [f32; 3]) {

        for mesh in &mut self.meshes {
            mesh.move_mesh(translation);
        }

    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn add_model(&mut self, model: Model) {
        for mesh in model.meshes {
            self.meshes.push(mesh);
        }
    }

    pub fn build_model(&mut self) -> (Vec<Vertex>, Vec<u16>) {

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let mut current_index = 0;

        for mesh in self.meshes.iter() {

            for vertex in mesh.vertices.iter() {
                vertices.push(vertex.clone());
            }

            for index in mesh.indices.iter() {
                indices.push(index + current_index);
            }

            current_index += mesh.vertices.len() as u16;
        }

        //println!("Model built:");
        //println!("  Vertices: {:#?}", vertices);
        //println!("  Indices: {:?}", indices);

        return (vertices, indices);
    }
}

//====================================================================

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
impl Mesh {
    pub fn move_mesh(&mut self, translation: [f32; 3]) {
        for vertex in &mut self.vertices {
            vertex.pos[0] += translation[0];
            vertex.pos[1] += translation[1];
            vertex.pos[2] += translation[2];
        }
    }
}

//====================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                //Pos
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                //Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ],
        }   
    }
}

//====================================================================
