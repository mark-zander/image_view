// Vertex shader

struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(2) @binding(0)
var<uniform> camera: CameraUniform;

struct MeshDescriptor {
    quads_in_row: u32,  // number of vertexes in a row
    rows_of_quads: u32, // number of rows
    xoffset: f32,       // location of first x value
    yoffset: f32,       // location of first y value
    xscale: f32,        // x scale factor
    yscale: f32,        // y scale factor
    channel: i32,       // red, green or blue color channel
    zoffset: f32,
};

@group(1) @binding(0)
var<uniform> mesh_desc: MeshDescriptor;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) wire_tex: vec2<f32>,
    @location(1) image_tex: vec2<f32>,
};

// @group(1) @binding(1)
// var<uniform> pos: array<vec2<f32>, 6>;

// relative positions of triangles in a quad
const offsets = array<vec2<i32>, 6>(
    vec2<i32>(0, 0),
    vec2<i32>(1, 1),
    vec2<i32>(0, 1),
    vec2<i32>(0, 0),
    vec2<i32>(1, 0),
    vec2<i32>(1, 1)
);

@vertex
fn vs_main(
    @builtin(vertex_index) index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    // number of vertexes in a quad, 2 triangles, 6 vertexes
    let quadsize = 6u;
    let iquad = index / quadsize;
    let quadvert = index % quadsize;
    let pos = vec2<i32>(i32(iquad % mesh_desc.quads_in_row),
        i32(iquad / mesh_desc.quads_in_row));
    var offset: vec2<i32>;
    switch quadvert {
        case 0u { offset = vec2<i32>(0, 0); }
        case 1u { offset = vec2<i32>(1, 1); }
        case 2u { offset = vec2<i32>(0, 1); }
        case 3u { offset = vec2<i32>(0, 0); }
        case 4u { offset = vec2<i32>(1, 0); }
        case 5u { offset = vec2<i32>(1, 1); }
        default { offset = vec2<i32>(0, 0); }
    }
    // let tcoords = pos + offset;
    // let coords = vec2<f32>(f32(tcoords.x), f32(tcoords.y));
    let coords = vec2<f32>(pos + offset);

    out.wire_tex = coords;

    out.image_tex.x = coords.x / f32(mesh_desc.quads_in_row);
    out.image_tex.y = 1.0 - coords.y / f32(mesh_desc.rows_of_quads);

    let x = coords.x * mesh_desc.xscale + mesh_desc.xoffset;
    let y = coords.y * mesh_desc.yscale + mesh_desc.yoffset;
    let dim = textureDimensions(image_tex);
    let icoords = vec2<i32>(
        i32(coords.x * f32(dim.x) / f32(mesh_desc.quads_in_row + 1u) + 0.5),
        i32((f32(mesh_desc.rows_of_quads) - coords.y) * f32(dim.y)
            / f32(mesh_desc.rows_of_quads + 1u) + 0.5)
    );

    let rgba = textureLoad(image_tex, icoords, 0);
    var z: f32;
    switch mesh_desc.channel {
        case 0 { z = sqrt(dot(rgba.rgb, rgba.rgb)) / 3.0; }
        case 1 { z = rgba.r; }
        case 2 { z = rgba.g; }
        case 3 { z = rgba.b; }
        default { z = sqrt(dot(rgba.rgb, rgba.rgb)) / 3.0; }
    }
    // let z = sqrt(dot(rgba.rgb, rgba.rgb)) / 3.0;
    out.clip_position = camera.view_proj * vec4<f32>(x, y, z, 1.0);

    return out;
}

// Fragment shader

@group(0) @binding(0)
var image_tex: texture_2d<f32>;
@group(0)@binding(1)
var image_sampler: sampler;

// Hardware wire frame
@fragment
fn fs_wire(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);

}

@fragment
fn fs_color(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(image_tex, image_sampler, in.image_tex);
}

@fragment
fn fs_fill(in: VertexOutput) -> @location(0) vec4<f32> {
    var out: vec4<f32>;
    let rgba = textureSample(image_tex, image_sampler, in.image_tex);
    switch mesh_desc.channel {
        case 0 { out = rgba; }
        case 1 { out = vec4<f32>(rgba.r, 0.0, 0.0, 1.0); }
        case 2 { out = vec4<f32>(0.0, rgba.g, 0.0, 1.0); }
        case 3 { out = vec4<f32>(0.0, 0.0, rgba.b, 1.0); }
        default {
            let grey = sqrt(dot(rgba.rgb, rgba.rgb)) / 3.0;
            return vec4<f32>(grey, grey, grey, 1.0);
        }
    }
    return out;
}

@fragment
fn fs_grey(in: VertexOutput) -> @location(0) vec4<f32> {
    let rgba = textureSample(image_tex, image_sampler, in.image_tex);
    let grey = sqrt(dot(rgba.rgb, rgba.rgb)) / 3.0;
    return vec4<f32>(grey, grey, grey, 1.0);
}

@fragment
fn fs_red(in: VertexOutput) -> @location(0) vec4<f32> {
    let rgba = textureSample(image_tex, image_sampler, in.image_tex);
    return vec4<f32>(rgba.r, 0.0, 0.0, 1.0);
}

@fragment
fn fs_green(in: VertexOutput) -> @location(0) vec4<f32> {
    let rgba = textureSample(image_tex, image_sampler, in.image_tex);
    return vec4<f32>(0.0, rgba.g, 0.0, 1.0);
}

@fragment
fn fs_blue(in: VertexOutput) -> @location(0) vec4<f32> {
    let rgba = textureSample(image_tex, image_sampler, in.image_tex);
    return vec4<f32>(0.0, 0.0, rgba.b, 1.0);
}

