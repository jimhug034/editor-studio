// Image processing shaders for Editor Studio

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

struct AdjustmentParams {
    brightness: f32,   // -1.0 to 1.0
    contrast: f32,     // 0.0 to 2.0
    saturation: f32,   // 0.0 to 2.0
    _padding: f32,
}

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var sampler: sampler;
@group(0) @binding(2) var<uniform> params: AdjustmentParams;

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VertexOutput {
    // Full-screen quad (triangle strip)
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0)
    );
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(1.0, 0.0)
    );

    var out: VertexOutput;
    out.position = vec4<f32>(positions[vi], 0.0, 1.0);
    out.uv = uvs[vi];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(texture, sampler, in.uv);

    // Apply brightness
    let with_brightness = color.rgb + params.brightness;

    // Apply contrast
    let with_contrast = (with_brightness - 0.5) * params.contrast + 0.5;

    // Apply saturation (convert to grayscale and mix)
    let gray = dot(with_contrast, vec3<f32>(0.299, 0.587, 0.114));
    let with_saturation = mix(gray, with_contrast, params.saturation);

    return vec4<f32>(with_saturation, color.a);
}
