import * as exe from "./mine_wasm.js";

async function run(memory: WebAssembly.Memory) {
  await exe.default(/* module */ undefined, memory);
  exe.main();
}

self.onmessage = (e: MessageEvent<WebAssembly.Memory>) => {
  const memory = e.data;
  run(memory).catch((e) => console.error(e));
};
