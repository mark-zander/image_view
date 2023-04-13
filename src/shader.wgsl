// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    // @location(0) color: vec3<f32>,
    @location(0) texcoord: vec2<f32>
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    // var pos = array<vec2<f32>, 6>(
    //    vec2<f32>(-1.0, -1.0),
    //    vec2<f32>( 1.0, -1.0),
    //    vec2<f32>(-1.0,  1.0),
    //    vec2<f32>(-1.0,  1.0),
    //    vec2<f32>( 1.0, -1.0),
    //    vec2<f32>( 1.0,  1.0)
    // );
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(-0.5, -0.5),
        vec2<f32>( 0.5, -0.5),
        vec2<f32>(-0.5,  0.5),
        vec2<f32>(-0.5,  0.5),
        vec2<f32>( 0.5, -0.5),
        vec2<f32>( 0.5,  0.5)
    );
    var color = array<vec3<f32>, 6>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(1.0, 1.0, 0.0),
        vec3<f32>(1.0, 1.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
    );
    var texcoord = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0)
    );

    var out: VertexOutput;
    // let x = f32(1 - i32(in_vertex_index)) * 0.5;
    // let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
    // out.color = color[in_vertex_index];
    out.texcoord = texcoord[in_vertex_index];
    return out;
}

struct MeshDescriptor {
    quads_in_row: u32,  // number of vertexes in a row
    rows_of_quads: u32, // number of rows
    xoffset: f32,       // location of first x value
    yoffset: f32,       // location of first y value
    xscale: f32,        // x scale factor
    yscale: f32,        // y scale factor
    channel: u32,       // red, green or blue color channel
    nverts: u32,
};

@group(1) @binding(0)
var<uniform> mesh_desc: MeshDescriptor;

struct VertexOutput1 {
    @builtin(position) clip_position: vec4<f32>,
};

// @group(1) @binding(1)
// var<uniform> pos: array<vec2<f32>, 6>;

@vertex
fn vs_main1(
    @builtin(vertex_index) index: u32,
) -> VertexOutput1 {
    var out: VertexOutput1;
    // number of vertexes in a quad, 2 triangles, 6 vertexes
    let quadsize = 6u;
    // relative positions of triangles in a quad
    // let pos = array<vec2<f32>, 6>(
    //     vec2<f32>(0.0, 0.0),
    //     vec2<f32>(1.0, 1.0),
    //     vec2<f32>(0.0, 1.0),
    //     vec2<f32>(0.0, 0.0),
    //     vec2<f32>(1.0, 0.0),
    //     vec2<f32>(1.0, 1.0)
    // );
    var xp: f32;
    var yp: f32;
    let iquad = index / 6u;
    let quadvert = index % 6u;
    let ix = iquad % mesh_desc.quads_in_row;
    let iy = iquad / mesh_desc.rows_of_quads;
    switch quadvert {
        case 0u { xp = 0.0; yp = 0.0; }
        case 1u { xp = 1.0; yp = 1.0; }
        case 2u { xp = 0.0; yp = 1.0; }
        case 3u { xp = 0.0; yp = 0.0; }
        case 4u { xp = 1.0; yp = 0.0; }
        case 5u { xp = 1.0; yp = 1.0; }
        default { xp = 0.0; yp = 0.0; }
    }
    out.clip_position.x = (f32(ix) + xp) * mesh_desc.xscale + mesh_desc.xoffset;
    out.clip_position.y = (f32(iy) + yp) * mesh_desc.yscale + mesh_desc.yoffset;
    out.clip_position.z = 0.0;
    out.clip_position.w = 1.0;

    // out.clip_position.x = (f32(ix) + pos[quadvert].x) * mesh_desc.xscale;
    // out.clip_position.y = (f32(iy) + pos[quadvert].y) * mesh_desc.yscale;
    // out.clip_position.z = 0.0;

    return out;
}

// Fragment shader

@group(0) @binding(0)
var image_tex: texture_2d<f32>;
@group(0)@binding(1)
var image_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(image_tex, image_sampler, in.texcoord);
}

// Hardware wire frame
@fragment
fn fs_wire(in: VertexOutput1) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);

}
