import * as exe from "./mine_wasm.js";

(self as any).send_to_host = (func: string, args: any[], ret: number) => {
  const obj: exe.Msg = { func, args, ret };
  self.postMessage(obj);
};

async function run(memory: WebAssembly.Memory) {
  await exe.default(/* module */ undefined, memory);
  exe.main();
}

self.onmessage = (e: MessageEvent<WebAssembly.Memory>) => {
  const memory = e.data;
  run(memory).catch((e) => console.error(e));
};
