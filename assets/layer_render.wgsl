//https://github.com/bevyengine/bevy/blob/c2da7800e3671ad92e775529070a814d0bc2f5f8/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct Blend {
    blend: f32,
    blend_mode: f32, // See the BlendMode struct in clip/clip_layer.rs for the meaning
}

@group(1) @binding(0)
var<uniform> blend: Blend;
@group(1) @binding(1)
var texture_a: texture_2d<f32>;
@group(1) @binding(2)
var our_sampler_a: sampler;
@group(1) @binding(3)
var texture_b: texture_2d<f32>;
@group(1) @binding(4)
var our_sampler_b: sampler;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    //var output_color = vec4<f32>(uniform_data.blend, sin(uniform_data.blend+input.uv.x*10.), 1.0, 1.0);
    // TODO: Figure out why uv coords are flipped
    var uv = vec2<f32>(1., 0.) + input.uv * vec2<f32>(-1., 1.);

    var output_color = vec4<f32>(0.);
    if (blend.blend_mode == 0.) { // Normal
        output_color = textureSample(texture_b, our_sampler_b, uv) * blend.blend;
    } else if (blend.blend_mode == 1.) { // Mix
        output_color = mix(
            textureSample(texture_a, our_sampler_a, uv),
            textureSample(texture_b, our_sampler_b, uv),
            blend.blend
        );
    } else if (blend.blend_mode == 2.) { // Multiply
        output_color = textureSample(texture_a, our_sampler_a, uv) * textureSample(texture_b, our_sampler_b, uv);
    } else if (blend.blend_mode == 3.) { // Screen
        output_color = 1. - (1.-textureSample(texture_a, our_sampler_a, uv)) * (1.-textureSample(texture_b, our_sampler_b, uv));
    } else if (blend.blend_mode == 4.) { // Add
        output_color = textureSample(texture_a, our_sampler_a, uv) + textureSample(texture_b, our_sampler_b, uv) * blend.blend;
    } else if (blend.blend_mode == 5.) { // Subtract
        output_color = textureSample(texture_a, our_sampler_a, uv) - textureSample(texture_b, our_sampler_b, uv) * blend.blend;
    } else if (blend.blend_mode == 6.) { // Difference
        output_color = textureSample(texture_b, our_sampler_b, uv) - textureSample(texture_a, our_sampler_a, uv) * blend.blend;
    }

    return output_color;
}
