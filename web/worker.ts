// Note that the specific exe here doesn't matter, we just need the same types found in all of them.
import type * as exe from "./exe/basicdd/basicdd.js";

(self as any).send_to_host = (func: string, args: any[], retAddr: number) => {
  const obj: exe.Msg = { func, args, retAddr };
  self.postMessage(obj);
};

async function run(module: string, memory: WebAssembly.Memory) {
  const exe = await import(module);
  await exe.default(/* module */ undefined, memory);
  exe.main();
}

export interface StartMessage {
  module: string;
  memory: WebAssembly.Memory;
}

self.onmessage = (e: MessageEvent<StartMessage>) => {
  const { module, memory } = e.data;
  run(module, memory).catch((e) => console.error(e));
};
