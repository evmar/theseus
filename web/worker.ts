import * as exe from "./basicdd.js";

(self as any).send_to_host = (func: string, args: any[], retAddr: number) => {
  const obj: exe.Msg = { func, args, retAddr };
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
