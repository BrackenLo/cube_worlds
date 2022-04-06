//====================================================================

use wgpu::util::DeviceExt;

use super::{model, voxel};

//====================================================================

// pub const TOTAL_CHUNKS_X: u16 = 10;
// pub const TOTAL_CHUNKS_Y: u16 = 1;
// pub const TOTAL_CHUNKS_Z: u16 = 10;

pub const CHUNK_WIDTH: u8 = 10;
pub const CHUNK_HEIGHT: u8 = 10;
pub const CHUNK_DEPTH: u8 = 10;

pub const CHUNK_SPAWN_RANGE: u8 = 5;

//MAX CHUNK WITH MUST NOT EXCEED 31

//Max chunk w/h/d = 0..31   -> u5

pub struct ChunkVoxels([[[Option<voxel::Voxel>; CHUNK_DEPTH as usize]; CHUNK_HEIGHT as usize]; CHUNK_WIDTH as usize]);

//====================================================================

pub struct ChunkCollection {
    pub chunks: std::collections::HashMap<glam::IVec3, Chunk>,
}
impl<'a> ChunkCollection {
    pub fn new() -> Self {
        let chunks = std::collections::HashMap::new();


        Self {
            chunks,
        }
    }

    pub fn spawn_chunks_in_range(&mut self, device: &wgpu::Device, pos: glam::IVec3) {

        let start_pos = glam::IVec3::new(
            pos.x - CHUNK_SPAWN_RANGE as i32,
            0,
            pos.z - CHUNK_SPAWN_RANGE as i32,
        );

        let end_pos = glam::IVec3::new(
            pos.x + CHUNK_SPAWN_RANGE as i32,
            1,
            pos.z + CHUNK_SPAWN_RANGE as i32,
        );

        for x in start_pos.x..end_pos.x {
            //println!("x = {}", {x});
            for y in start_pos.y..end_pos.y {
                //println!("y = {}", {y});
                for z in start_pos.z..end_pos.z {
                    //println!("z = {}", {z});
                    self.spawn_chunk(device, glam::IVec3::new(x, y, z));
                }
            }
        }
    }



    pub fn spawn_chunk(&mut self, device: &wgpu::Device, pos: glam::IVec3) {
        if !self.chunks.contains_key(&pos) {

            self.chunks.insert(
                pos,
                Chunk::new(device, pos),
            );
        }
        else {
            println!("Trying to spawn pre-existing chunk");
        }
    }
}

//====================================================================

pub struct Chunk {

    voxels: ChunkVoxels,

    //voxels: std::collections::HashMap<u16, voxel::Voxel>,
    //pub voxel_count: u32,

    //pub instance_buffer: wgpu::Buffer,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,

    //vertices: Vec<u16>,
}

impl<'a> Chunk {
    pub fn new(device: &wgpu::Device, chunk_pos: glam::IVec3) -> Self {

        let voxels = ChunkVoxels([[[
            Some(voxel::Voxel::Grass); CHUNK_DEPTH as usize]; CHUNK_HEIGHT as usize]; CHUNK_WIDTH as usize
        ]);

        let (vertex_buffer, index_buffer, index_count) = Self::build_chunk_model(device, chunk_pos, &voxels);

        
        Self {
            voxels,
            vertex_buffer,
            index_buffer,
            index_count,
        }
        
    }

    pub fn rebuild_chunk_model(
        &mut self, 
        device: &wgpu::Device,
        chunk_pos: glam::IVec3,
    ) {
        let (
            vertex_buffer, 
            index_buffer, 
            index_count
        ) = Self::build_chunk_model(device, chunk_pos, &self.voxels);

        self.vertex_buffer = vertex_buffer;
        self.index_buffer = index_buffer;
        self.index_count = index_count;
    }

    pub fn build_chunk_model(
        device: &wgpu::Device,
        chunk_pos: glam::IVec3,
        voxels: &ChunkVoxels,
    ) -> (wgpu::Buffer, wgpu::Buffer, u32) {

        let mut chunk_model = model::Model::default();

        //println!("Total size: ({}, {}, {})", voxels.0.len())

        for x in 0..voxels.0.len() {
            for y in 0..voxels.0[x].len() {
                for z in 0..voxels.0[x][y].len() {

                    if voxels.0[x][y][z].is_none() {
                        continue;
                    }

                    let mut draw_left = false;
                    let mut draw_right = false;

                    let mut draw_top = false;
                    let mut draw_bottom = false;

                    let mut draw_front = false;
                    let mut draw_back = false;


                    if x == 0 && x == CHUNK_WIDTH as usize - 1 {
                        draw_left = true;
                        draw_right = true;
                    }
                    else if x == 0 {
                        draw_left = true;

                        if voxels.0[x + 1][y][z].is_none() {
                            draw_right = true;
                        }
                    }
                    else if x == CHUNK_WIDTH as usize - 1 {
                        draw_right = true;

                        if voxels.0[x - 1][y][z].is_none() {
                            draw_left = true;
                        }
                    }
                    else {
                        if voxels.0[x + 1][y][z].is_none() {
                            draw_right = true;
                        }
                        if voxels.0[x - 1][y][z].is_none() {
                            draw_left = true;
                        }
                    }

                    if y == 0 && y == CHUNK_HEIGHT as usize - 1 {
                        draw_bottom = true;
                        draw_top = true;
                    }
                    else if y == 0 {
                        draw_bottom = true;

                        if voxels.0[x][y + 1][z].is_none() {
                            draw_top = true;
                        }
                    }
                    else if y == CHUNK_HEIGHT as usize - 1 {
                        draw_top = true;

                        if voxels.0[x][y - 1][z].is_none() {
                            draw_bottom = true;
                        }
                    }
                    else {
                        if voxels.0[x][y + 1][z].is_none() {
                            draw_top = true;
                        }
                        if voxels.0[x][y - 1][z].is_none() {
                            draw_bottom = true;
                        }
                    }

                    if z == 0 && z == CHUNK_DEPTH as usize - 1 {
                        draw_back = true;
                        draw_front = true;
                    }
                    else if z == 0 {
                        draw_back = true;

                        if voxels.0[x][y][z + 1].is_none() {
                            draw_front = true;
                        }
                    }
                    else if z == CHUNK_DEPTH as usize - 1 {
                        draw_front = true;

                        if voxels.0[x][y][z - 1].is_none() {
                            draw_back = true;
                        }
                    }
                    else {
                        if voxels.0[x][y][z + 1].is_none() {
                            draw_front = true;
                        }
                        if voxels.0[x][y][z - 1].is_none() {
                            draw_back = true;
                        }
                    }

                    let voxel_color = voxels.0[x][y][z].unwrap().get_color();
                    let mut voxel_model = model::Model::default();

                    if draw_left    {voxel_model.add_mesh(voxel::Voxel::get_side(voxel::Side::Left, voxel_color));}
                    if draw_right   {voxel_model.add_mesh(voxel::Voxel::get_side(voxel::Side::Right, voxel_color));}
                    if draw_top     {voxel_model.add_mesh(voxel::Voxel::get_side(voxel::Side::Top, voxel_color));}
                    if draw_bottom  {voxel_model.add_mesh(voxel::Voxel::get_side(voxel::Side::Bottom, voxel_color));}
                    if draw_front   {voxel_model.add_mesh(voxel::Voxel::get_side(voxel::Side::Front, voxel_color));}
                    if draw_back    {voxel_model.add_mesh(voxel::Voxel::get_side(voxel::Side::Back, voxel_color));}

                    voxel_model.move_model([
                        x as f32 * voxel::VOXEL_WIDTH,
                        y as f32 * voxel::VOXEL_HEIGHT,
                        z as f32 * voxel::VOXEL_DEPTH,
                    ]);

                    chunk_model.add_model(voxel_model);
                }
            }
        }

        chunk_model.move_model([
            chunk_pos.x as f32 * voxel::VOXEL_WIDTH * CHUNK_WIDTH as f32,
            chunk_pos.y as f32 * voxel::VOXEL_HEIGHT * CHUNK_HEIGHT as f32,
            chunk_pos.z as f32 * voxel::VOXEL_DEPTH * CHUNK_DEPTH as f32,
        ]);

        let (vertices, indices) = chunk_model.build_model();


        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Chunk Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        return (vertex_buffer, index_buffer, indices.len() as u32);

        //build the new model here and put its vertices and indicies in a buffer
        //and work it into the shader somehow. Also rework draw chunk trait :D

    }

}

//====================================================================

pub trait DrawChunk<'a> {
    fn draw_chunks(&mut self, chunks: &'a ChunkCollection);
    fn draw_chunk(&mut self, chunk: &'a Chunk);
}

impl<'a, 'b> DrawChunk<'b> for wgpu::RenderPass<'a> 
where 
    'b: 'a
{
    fn draw_chunks(
        &mut self, 
        chunks: &'b ChunkCollection
    ) {

        //self.set_vertex_buffer(0, chunks.vertex_buffer.slice(..));
        //self.set_index_buffer(chunks.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        for chunk in chunks.chunks.values() {
            self.draw_chunk(chunk);
        }
    }

    fn draw_chunk(
        &mut self,
        chunk: &'b Chunk,
    ) {
        //println!("Drawing chunk with {} indices and {} voxels", indices.end, chunk.voxel_count);
        //self.set_vertex_buffer(1, chunk.instance_buffer.slice(..));
        //self.draw_indexed(indices, 0, 0..chunk.voxel_count);

        self.set_vertex_buffer(0, chunk.vertex_buffer.slice(..));
        self.set_index_buffer(chunk.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        self.draw_indexed(0..chunk.index_count, 0, 0..1);
    }
}

//====================================================================