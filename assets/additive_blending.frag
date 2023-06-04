layout(location = 0) in vec2 vUv;

layout(location = 0) out vec4 gl_FragColor;

layout(set = 0, binding = 0) uniform texture2D tDiffuse_texture;
layout(set = 0, binding = 1) uniform sampler tDiffuse_sampler;
layout(set = 0, binding = 2) uniform texture2D tAdd_texture;
layout(set = 0, binding = 3) uniform sampler tAdd_sampler;

void main() {
    vec4 color = texture(sampler2D(tDiffuse_texture,tDiffuse_sampler), vUv );
    vec4 add = texture( sampler2D(tAdd_texture, tAdd_sampler), vUv );
    gl_FragColor = color + add;
}