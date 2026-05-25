const params = new Map();
let setParameterNative = null;

if (window.__JUCE__?.backend) {
  setParameterNative = window.__JUCE__.backend.getNativeFunction("setParameter");
}

function setParam(id, value) {
  const next = Math.max(0, Math.min(1, value));
  params.set(id, next);
  document.querySelectorAll(`[data-param="${id}"]`).forEach((element) => {
    element.style.setProperty("--value", next.toFixed(4));
    element.style.setProperty("--angle", `${Math.round(next * 300)}deg`);
    const valueLabel = element.querySelector("strong");
    if (valueLabel) valueLabel.textContent = String(Math.round(next * 100));
    if (element instanceof HTMLInputElement) element.value = String(next);
  });
  setParameterNative?.(id, next);
}

document.querySelectorAll("[data-param]").forEach((element) => {
  const id = element.dataset.param;
  if (element instanceof HTMLInputElement) {
    element.addEventListener("input", () => setParam(id, Number(element.value)));
  } else {
    element.addEventListener("pointerdown", (event) => {
      element.setPointerCapture(event.pointerId);
      const originY = event.clientY;
      const origin = params.get(id) ?? 0.5;
      const move = (moveEvent) => setParam(id, origin + (originY - moveEvent.clientY) / 260);
      const up = () => {
        element.removeEventListener("pointermove", move);
        element.removeEventListener("pointerup", up);
      };
      element.addEventListener("pointermove", move);
      element.addEventListener("pointerup", up);
    });
  }
});

window.__JUCE__?.backend?.addEventListener("parameterSnapshot", (snapshot) => {
  snapshot.forEach(({ id, value }) => setParam(id, value));
});

window.__JUCE__?.backend?.addEventListener("meters", ([input, output, wet]) => {
  document.documentElement.style.setProperty("--meter-input", input);
  document.documentElement.style.setProperty("--meter-output", output);
  document.documentElement.style.setProperty("--meter-wet", wet);
});

const canvas = document.getElementById("icefield");
const gl = canvas.getContext("webgl", { alpha: true, antialias: true });

if (gl) {
  const vertex = `
    attribute vec2 position;
    void main() { gl_Position = vec4(position, 0.0, 1.0); }
  `;
  const fragment = `
    precision mediump float;
    uniform vec2 resolution;
    uniform float time;
    float hash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
    void main() {
      vec2 uv = gl_FragCoord.xy / resolution.xy;
      float frost = hash(floor((uv + time * 0.012) * 42.0));
      float shimmer = smoothstep(0.72, 1.0, frost) * 0.28;
      gl_FragColor = vec4(0.42, 0.94, 0.92, shimmer);
    }
  `;
  const shader = (type, source) => {
    const s = gl.createShader(type);
    gl.shaderSource(s, source);
    gl.compileShader(s);
    return s;
  };
  const program = gl.createProgram();
  gl.attachShader(program, shader(gl.VERTEX_SHADER, vertex));
  gl.attachShader(program, shader(gl.FRAGMENT_SHADER, fragment));
  gl.linkProgram(program);
  gl.useProgram(program);
  const buffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
  const position = gl.getAttribLocation(program, "position");
  gl.enableVertexAttribArray(position);
  gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 0, 0);
  const resolution = gl.getUniformLocation(program, "resolution");
  const time = gl.getUniformLocation(program, "time");
  const render = (now) => {
    const ratio = window.devicePixelRatio || 1;
    canvas.width = Math.floor(canvas.clientWidth * ratio);
    canvas.height = Math.floor(canvas.clientHeight * ratio);
    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.uniform2f(resolution, canvas.width, canvas.height);
    gl.uniform1f(time, now * 0.001);
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
    requestAnimationFrame(render);
  };
  requestAnimationFrame(render);
}

[
  ["ice_size", 0.55],
  ["ancient_depth", 0.48],
  ["coldness", 0.62],
  ["wind", 0.18],
  ["frozen_harmonics", 0.24],
  ["storm", 0.2],
  ["distance", 0.42],
  ["input", 0.75],
  ["mix", 0.34],
  ["output", 0.75],
  ["freeze", 0],
].forEach(([id, value]) => setParam(id, value));
