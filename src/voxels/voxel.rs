//====================================================================

use super::model;

//====================================================================

pub const VOXEL_WIDTH: f32 = 0.3;
pub const VOXEL_HEIGHT: f32 = 0.3;
pub const VOXEL_DEPTH: f32 = 0.3;

//pub const VOXEL_SIZE: glam::Vec3 = glam::Vec3::new(VOXEL_WIDTH, VOXEL_HEIGHT, VOXEL_DEPTH,);

pub const HALF_VOXEL_SIZE: glam::Vec3 = glam::Vec3::new(
    VOXEL_WIDTH / 2.,
    VOXEL_HEIGHT / 2.,
    VOXEL_DEPTH / 2.,
);

//====================================================================

pub enum Side {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right
}

//====================================================================

#[derive(Clone, Copy)]
pub enum Voxel {
    Air,
    Grass,
    Stone,
}

impl Voxel {

    pub fn get_color(&self) -> [f32; 3] {

        let mut rng = rand::thread_rng();

        match self {
            Voxel::Air => [1., 1., 1.,],
            Voxel::Grass => {

                [0., rand::Rng::gen_range(&mut rng, 0.3..1f32) , 0.,]
            },
            Voxel::Stone => [0.3, 0.3, 0.3,],
        }
    }

    pub fn get_side(dir: Side, color: [f32; 3]) -> model::Mesh {
        match dir {
            Side::Top => model::Mesh {
                vertices: Vec::from([
                        model::Vertex {pos: Self::V_0, color },
                        model::Vertex {pos: Self::V_4, color },
                        model::Vertex {pos: Self::V_7, color },
                        model::Vertex {pos: Self::V_3, color },
                    ]),
                indices: Vec::from(Self::DEFAULT_INDICES),
            },
            Side::Bottom => model::Mesh {
                vertices: Vec::from([
                        model::Vertex {pos: Self::V_2, color },
                        model::Vertex {pos: Self::V_6, color },
                        model::Vertex {pos: Self::V_5, color },
                        model::Vertex {pos: Self::V_1, color },
                    ]),
                indices: Vec::from(Self::DEFAULT_INDICES),
            },

            Side::Front => model::Mesh {
                vertices: Vec::from([
                        model::Vertex {pos: Self::V_4, color },
                        model::Vertex {pos: Self::V_5, color },
                        model::Vertex {pos: Self::V_6, color },
                        model::Vertex {pos: Self::V_7, color },
                    ]),
                indices: Vec::from(Self::DEFAULT_INDICES),
            },
            Side::Back => model::Mesh {
                vertices: Vec::from([
                        model::Vertex {pos: Self::V_3, color },
                        model::Vertex {pos: Self::V_2, color },
                        model::Vertex {pos: Self::V_1, color },
                        model::Vertex {pos: Self::V_0, color },
                    ]),
                indices: Vec::from(Self::DEFAULT_INDICES),
            },

            Side::Left => model::Mesh {
                vertices: Vec::from([
                        model::Vertex {pos: Self::V_0, color },
                        model::Vertex {pos: Self::V_1, color },
                        model::Vertex {pos: Self::V_5, color },
                        model::Vertex {pos: Self::V_4, color },
                    ]),
                indices: Vec::from(Self::DEFAULT_INDICES),
            },
            Side::Right => model::Mesh {
                vertices: Vec::from([
                        model::Vertex {pos: Self::V_7, color },
                        model::Vertex {pos: Self::V_6, color },
                        model::Vertex {pos: Self::V_2, color },
                        model::Vertex {pos: Self::V_3, color },
                    ]),
                indices: Vec::from(Self::DEFAULT_INDICES),
            },
        }
    }

    pub const V_0: [f32; 3] = [-HALF_VOXEL_SIZE.x,  HALF_VOXEL_SIZE.y, -HALF_VOXEL_SIZE.z]; //0
    pub const V_1: [f32; 3] = [-HALF_VOXEL_SIZE.x, -HALF_VOXEL_SIZE.y, -HALF_VOXEL_SIZE.z]; //1
    pub const V_2: [f32; 3] = [ HALF_VOXEL_SIZE.x, -HALF_VOXEL_SIZE.y, -HALF_VOXEL_SIZE.z]; //2
    pub const V_3: [f32; 3] = [ HALF_VOXEL_SIZE.x,  HALF_VOXEL_SIZE.y, -HALF_VOXEL_SIZE.z]; //3
    pub const V_4: [f32; 3] = [-HALF_VOXEL_SIZE.x,  HALF_VOXEL_SIZE.y,  HALF_VOXEL_SIZE.z]; //4
    pub const V_5: [f32; 3] = [-HALF_VOXEL_SIZE.x, -HALF_VOXEL_SIZE.y,  HALF_VOXEL_SIZE.z]; //5
    pub const V_6: [f32; 3] = [ HALF_VOXEL_SIZE.x, -HALF_VOXEL_SIZE.y,  HALF_VOXEL_SIZE.z]; //6
    pub const V_7: [f32; 3] = [ HALF_VOXEL_SIZE.x,  HALF_VOXEL_SIZE.y,  HALF_VOXEL_SIZE.z]; //7

    pub const DEFAULT_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

}

//====================================================================

///
/// 0 00000 00000 00000
/// ?   x     y     z
///   31744  922   31
/// 
/// each value can go from 0 -> 31
#[derive(Default)]
pub struct LocalCoord (u16);
#[allow(unused)]
impl LocalCoord {

    //--------------------------------------------------

    const X_BITWISE: u16 = 31744;
    const Y_BITWISE: u16 = 992;
    const Z_BITWISE: u16 = 31;

    //--------------------------------------------------

    pub fn new(x: u8, y: u8, z: u8) -> Self {

        let x = Self::clamp_value(x) << 10;
        let y = Self::clamp_value(y) << 5;
        let z = Self::clamp_value(z);

        Self(x | y | z)
    }

    pub fn from_raw(raw: u16) -> Self {
        Self(
            raw & !32768
        )
    }

    pub fn to_raw(x: u8, y: u8, z: u8) -> u16 {
        let x = Self::clamp_value(x) << 10;
        let y = Self::clamp_value(y) << 5;
        let z = Self::clamp_value(z);

        x + y + z
    }

    pub fn get_raw(&self) -> u16 {
        return self.0;
    }

    //--------------------------------------------------

    pub fn x(&self) -> u8 {
        return ((self.0 & Self::X_BITWISE) >> 10) as u8;
    }
    pub fn y(&self) -> u8 {
        return ((self.0 & Self::Y_BITWISE) >> 5) as u8;
    }
    pub fn z(&self) -> u8 {
        return (self.0 & Self::Z_BITWISE) as u8;
    }

    pub fn xyz(&self) -> (u8, u8, u8) {
        return ( self.x(), self.y(), self.z() )
    }

    //--------------------------------------------------

    pub fn set_x(&mut self, value: u8) {

        let value = Self::clamp_value(value);
        let value = value << 10;

        self.0 &= !Self::X_BITWISE;
        self.0 += value;
    }

    pub fn set_y(&mut self, value: u8) {
        let value = Self::clamp_value(value);
        let value = value << 5;

        self.0 &= !Self::Y_BITWISE;
        self.0 += value;
    }

    pub fn set_z(&mut self, value: u8) {
        let value = Self::clamp_value(value);

        self.0 &= !Self::Z_BITWISE;
        self.0 += value;
    }

    //--------------------------------------------------

    pub fn add_x(&mut self, value: u8) {
        self.set_x(value + self.x());
    }
    
    pub fn add_y(&mut self, value: u8) {
        self.set_y(value + self.y());
    }

    pub fn add_z(&mut self, value: u8) {
        self.set_z(value + self.z());
    }

    //--------------------------------------------------

    pub fn sub_x(&mut self, value: u8) {
        self.set_x(self.x() - value);
    }
    
    pub fn sub_y(&mut self, value: u8) {
        self.set_y(self.y() - value);
    }

    pub fn sub_z(&mut self, value: u8) {
        self.set_z(self.z() - value);
    }

    //--------------------------------------------------

    fn clamp_value(value: u8) -> u16 {
        if value > 31 {
            println!("User Error: value submitted to LocalVoxelCoord clamp_value() exceeds 31. Data will be lost");
        }
        return value as u16 & 31;
    }
}

//====================================================================