import init, { WasmRenderer, initThreadPool } from "./pkg/oxide.js";

let renderer = null;

async function setup() {
  await init();
  await initThreadPool(Math.max(1, navigator.hardwareConcurrency - 2));
  renderer = new WasmRenderer();
  postMessage({ type: "ready" });
}

self.onmessage = async (e) => {
  const { type, id, params } = e.data;

  if (type === "render") {
    if (!renderer) return;
    const { w, h, fov, cam_x, cam_y, cam_z, target_x, target_y, target_z, samples, termProb } = params;
    const t0 = performance.now();
    const rgba = renderer.render(w, h, fov, cam_x, cam_y, cam_z, target_x, target_y, target_z, samples, termProb);
    const dt = performance.now() - t0;
    postMessage({ type: "frame", id, width: w, height: h, dt, rgba: rgba.buffer }, [rgba.buffer]);
  }
};

setup();
