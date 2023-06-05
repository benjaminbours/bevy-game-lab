#version 450

layout(location = 0) in vec2 vUv;

layout(location = 0) out vec4 gl_FragColor;


// layout(set = 1, binding = 0) uniform CustomMaterial {
//     vec4 Color;
// };

// layout(set = 1, binding = 1) uniform texture2D CustomMaterial_texture;
// layout(set = 1, binding = 2) uniform sampler CustomMaterial_sampler;

layout(set = 1, binding = 0) uniform vec2 light_position;
layout(set = 1, binding = 1) uniform float exposure;
layout(set = 1, binding = 2) uniform float decay;
layout(set = 1, binding = 3) uniform float density;
layout(set = 1, binding = 4) uniform float weight;
layout(set = 1, binding = 5) uniform int samples;
layout(set = 1, binding = 6) uniform texture2D tDiffuse_texture;
layout(set = 1, binding = 7) uniform sampler tDiffuse_sampler;
const int MAX_SAMPLES = 1000;

void main(){
  
  vec2 texCoord = vUv;
  // Calculate vector from pixel to light source in screen space
  vec2 deltaTextCoord = texCoord - light_position;
  // Divide by number of samples and scale by control factor
  deltaTextCoord *= 1.0 / float(samples) * density;
  // Store initial sample
  vec4 color = texture(sampler2D(tDiffuse_texture,tDiffuse_sampler), texCoord);
  // set up illumination decay factor
  float illuminationDecay = 1.0;
  
  // evaluate the summation for samples number of iterations up to 100
  for(int i=0; i < MAX_SAMPLES; i++){
    // work around for dynamic number of loop iterations
    if(i == samples){
      break;
    }
    
    // step sample location along ray
    texCoord -= deltaTextCoord;
    // retrieve sample at new location
    vec4 sampleVec = texture(sampler2D(tDiffuse_texture,tDiffuse_sampler), texCoord);
    // apply sample attenuation scale/decay factors
    sampleVec *= illuminationDecay * weight;
    // accumulate combined color
    color += sampleVec;
    // update exponential decay factor
    illuminationDecay *= decay;
  
  }
  // output final color with a further scale control factor
  gl_FragColor = color * exposure;
}