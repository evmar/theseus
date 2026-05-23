import * as exe from "./mine_wasm.js";

class Host implements exe.WasmHost {
  consoleDom = document.createElement("pre");
  consoleOutput = new ArrayBuffer(0, { maxByteLength: 10 << 10 });
  window_: HTMLCanvasElement | undefined;

  constructor(public wasmMemory: WebAssembly.Memory) {
    if (!window.SharedArrayBuffer) {
      alert("SharedArrayBuffer is not supported");
    }

    this.consoleDom.id = "console";
    document.body.appendChild(this.consoleDom);
  }

  onMessage(e: MessageEvent<exe.Msg>) {
    const msg = e.data;
    console.log("msg", msg);
    const ret = (this as any)[msg.func](...msg.args);
    if (msg.ret) {
      if (ret == 0) throw new Error();
      const ints = new Int32Array(this.wasmMemory.buffer, msg.ret, 1);
      ints[0] = ret;
      console.log("notify");
      Atomics.notify(ints, 0, 1);
    }
  }

  console_write(ptr: number, len: number): void {
    const inBuf = new Uint8Array(this.wasmMemory.buffer, ptr, len);
    const ofs = this.consoleOutput.byteLength;
    this.consoleOutput.resize(ofs + len);
    const outBuf = new Uint8Array(this.consoleOutput, ofs, len);
    outBuf.set(inBuf);
    this.consoleDom.innerText = new TextDecoder().decode(this.consoleOutput);
  }

  create_surface(width: number, height: number): number {
    const surface = document.createElement("canvas");
    surface.width = width;
    surface.height = height;
    document.body.appendChild(surface);
    return 1;
  }

  create_window(title: string, width: number, height: number): number {
    this.window_ = document.createElement("canvas");
    this.window_.className = "window";
    this.window_.width = width;
    this.window_.height = height;
    document.body.appendChild(this.window_);
    return 1;
  }

  resize_window(id: number, width: number, height: number): void {
    this.window_!.width = width;
    this.window_!.height = height;
  }
}

async function main() {
  const memory = new WebAssembly.Memory({
    initial: 32, // in units of 64KB pages, 2mb
    maximum: 1024, // 64mb
    shared: true,
  });

  const host = new Host(memory);
  const worker = new Worker("./worker.js", { type: "module" });
  worker.onmessage = (e) => host.onMessage(e);
  worker.postMessage(memory);
}
main().catch((e) => console.error(e));
