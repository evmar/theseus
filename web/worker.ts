// Note that the specific exe here doesn't matter, we just need the same types found in all of them.
import type * as exe from "./exe/basicdd/basicdd.js";

(self as any).send_to_host = (func: string, args: any[], retAddr: number) => {
  const obj: exe.Msg = { func, args, retAddr };
  self.postMessage(obj);
};

async function run(name: string, memory: WebAssembly.Memory) {
  const exe = await import(`./exe/${name}/${name}.js`);
  await exe.default(/* module */ undefined, memory);
  exe.main();
}

export interface StartMessage {
  name: string;
  memory: WebAssembly.Memory;
}

self.onmessage = (e: MessageEvent<StartMessage>) => {
  const { name, memory } = e.data;
  run(name, memory).catch((e) => console.error(e));
};
