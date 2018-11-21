import('./wasm/wasm_test').then((wasm) => {
  const canvas = document.querySelector('#canvas');
  const gl = canvas.getContext('webgl');

  let width = 0;
  let height = 0;
  const resizeCanvas = () => {
    width = canvas.width = canvas.offsetWidth;
    height = canvas.height = canvas.offsetHeight;
  };
  window.addEventListener('resize', resizeCanvas);
  resizeCanvas();

  let timePrev = 0;
  const state = wasm.newState(gl, width, height);
  const frame = (time) => {
    const delta = (timePrev === 0 ? 0 : time - timePrev) / 1000.0;
    wasm.step(state, delta, width, height);
    timePrev = time;
    window.requestAnimationFrame(frame);
  };
  window.requestAnimationFrame(frame);
});
