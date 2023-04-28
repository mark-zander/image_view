// Vertex shader

struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(2) @binding(0)
var<uniform> camera: CameraUniform;

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
    @location(0) wire_tex: vec2<f32>,
    @location(1) image_tex: vec2<f32>,
};

// @group(1) @binding(1)
// var<uniform> pos: array<vec2<f32>, 6>;

// relative positions of triangles in a quad
const pos = array<vec2<i32>, 6>(
    vec2<i32>(0, 0),
    vec2<i32>(1, 1),
    vec2<i32>(0, 1),
    vec2<i32>(0, 0),
    vec2<i32>(1, 0),
    vec2<i32>(1, 1)
);

@vertex
fn vs_main1(
    @builtin(vertex_index) index: u32,
) -> VertexOutput1 {
    var out: VertexOutput1;
    // number of vertexes in a quad, 2 triangles, 6 vertexes
    let quadsize = 6u;
    let iquad = index / quadsize;
    let quadvert = index % quadsize;
    let ix = iquad % mesh_desc.quads_in_row;
    let iy = iquad / mesh_desc.rows_of_quads;
    var xp: i32;
    var yp: i32;
    switch quadvert {
        case 0u { xp = 0; yp = 0; }
        case 1u { xp = 1; yp = 1; }
        case 2u { xp = 0; yp = 1; }
        case 3u { xp = 0; yp = 0; }
        case 4u { xp = 1; yp = 0; }
        case 5u { xp = 1; yp = 1; }
        default { xp = 0; yp = 0; }
    }
    // let xp = pos[quadvert].x;
    // let yp = pos[quadvert].y;

    // let jx = f32(ix) + xp;
    // let jy = f32(iy) + yp;
    let jx = i32(ix) + i32(xp);
    let jy = i32(iy) + i32(yp);
    let coords = vec2<f32>(f32(jx), f32(jy));
    let icoords = vec2<i32>(i32(jx), i32(jy));

    out.wire_tex = coords;

    out.image_tex.x = coords.x / f32(mesh_desc.quads_in_row);
    out.image_tex.y = 1.0 - coords.y / f32(mesh_desc.rows_of_quads);

    // out.clip_position.x = coords.x * mesh_desc.xscale + mesh_desc.xoffset;
    // out.clip_position.y = coords.y * mesh_desc.yscale + mesh_desc.yoffset;
    let x = coords.x * mesh_desc.xscale + mesh_desc.xoffset;
    let y = coords.y * mesh_desc.yscale + mesh_desc.yoffset;
    let dim = textureDimensions(image_tex);
    let jcoords = vec2<i32>(
        i32(coords.x * f32(dim.x) / f32(mesh_desc.quads_in_row)),
        i32((f32(mesh_desc.rows_of_quads) - coords.y) * f32(dim.y)
            / f32(mesh_desc.rows_of_quads))
    );
    // let jcoords = vec2<i32>(
    //     icoords.x * dim.x / i32(mesh_desc.quads_in_row),
    //     icoords.y * dim.y / i32(mesh_desc.rows_of_quads)
    // );
    let rgba = textureLoad(image_tex, jcoords, 0);
    let z = sqrt(dot(rgba.rgb, rgba.rgb)) / 3.0;
    out.clip_position = camera.view_proj * vec4<f32>(x, y, z, 1.0);

    return out;
}

// Fragment shader

@group(0) @binding(0)
var image_tex: texture_2d<f32>;
@group(0)@binding(1)
var image_sampler: sampler;

@fragment
fn fs_color(in: VertexOutput1) -> @location(0) vec4<f32> {
    return textureSample(image_tex, image_sampler, in.image_tex);
}

// Hardware wire frame
@fragment
fn fs_wire(in: VertexOutput1) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);

}
