#import bevy_pbr::mesh_bindings   mesh
#import bevy_pbr::mesh_functions  mesh_position_local_to_clip
#import bevy_pbr::mesh_view_bindings globals

struct CustomMaterial {
    color: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> material: CustomMaterial;

fn grassSwing(input: f32) -> f32 {
    // 将输入映射到[0, 1]范围，假设输入的范围是[-1, 1]
    let scaledInput: f32 = (input + 1.0) / 2.0;

    // 计算振幅，使用指数增长
    let amplitude: f32 = 2.0 * pow(2.0, scaledInput);

    // 使用正弦函数来模拟摇摆，可以根据需要调整频率
    let frequency: f32 = 2.0 * globals.time; // 调整频率
    let swingValue: f32 = amplitude * sin(frequency);

    // 返回摇摆值
    return swingValue;
}

struct Vertex {
    @location(0) position: vec3<f32>,
//    @location(1) color: vec4<f32>,
//    @location(1) blend_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // 计算摇摆偏移 dx = A * f(h) * sin(B * t) 这里的 A 是晃动最大幅度, B 是晃动速度, f(h) 则代表高度对幅度的影响函数
    let sway_offset = grassSwing(vertex.position.y) ;
    // 将草的位置进行摇摆
    var new_position = vec4<f32>(vertex.position, 1.0);
    new_position.x += sway_offset;

//    out.blend_color = vertex.color;
    out.clip_position = mesh_position_local_to_clip(
            mesh.model,
            new_position
        );
    out.blend_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    return out;
}

struct FragmentInput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
//    return material.color * input.blend_color;
    return input.blend_color;
}
