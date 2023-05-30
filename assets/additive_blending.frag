layout(location = 0) in vec2 vUv;

layout(location = 0) out vec4 gl_FragColor;

layout(set = 0, binding = 0) uniform texture2D tDiffuse_texture;
layout(set = 0, binding = 1) uniform sampler tDiffuse_sampler;
// layout(set = 1, binding = 0) uniform texture2D tDiffuse_texture;
// layout(set = 1, binding = 1) uniform sampler tDiffuse_sampler;
// layout(set = 1, binding = 1) uniform sampler tAdd;
// layout(set = 1, binding = 1) uniform sampler tAdd;
struct Settings {
    float intensity;
    float other;
};

layout(set = 0, binding = 2) uniform Settings settings;

void main() {
    vec4 color = texture(sampler2D(tDiffuse_texture,tDiffuse_sampler), vUv );
    // vec4 add = texture( tAdd, vUv );
    // gl_FragColor = color + add;
    gl_FragColor = color * vec4(settings.intensity, settings.other, 1., 1);
}