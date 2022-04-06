//====================================================================
//Vertex Shader

struct VertexIn {
    [[location(0)]] pos: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};

// struct InstanceIn {
//     [[location(1)]] transform_0: vec4<f32>;
//     [[location(2)]] transform_1: vec4<f32>;
//     [[location(3)]] transform_2: vec4<f32>;
//     [[location(4)]] transform_3: vec4<f32>;

//     [[location(5)]] color: vec3<f32>;
// };

//--------------------------------------------------

struct VertexOut {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
    
};

//--------------------------------------------------

struct CameraUniform {
    view_proj: mat4x4<f32>;
};
[[group(0), binding(0)]]
var<uniform> u_camera: CameraUniform;

//--------------------------------------------------

[[stage(vertex)]]
fn vs_main(
    vertex_in: VertexIn,
    //instance_in: InstanceIn,
) -> VertexOut {

    // let instance_transform = mat4x4<f32>(
    //     instance_in.transform_0,
    //     instance_in.transform_1,
    //     instance_in.transform_2,
    //     instance_in.transform_3,
    // );

    var out: VertexOut;

    out.clip_position = u_camera.view_proj * vec4<f32>(vertex_in.pos, 1.,);
    //out.clip_position = u_camera.view_proj * instance_transform * vec4<f32>(vertex_in.pos, 1.,);
    out.color = vertex_in.color;

    return out;
}

//====================================================================

[[stage(fragment)]]
fn fs_main(
    in: VertexOut,
) -> [[location(0)]] vec4<f32> {

    return vec4<f32>(in.color, 1.0);

}

//====================================================================