import * as exe from "./mine_wasm.js";

class MessageQueue {
  private messages: number[] = [];
  private waiter: ((value: number) => void) | undefined;

  poll(): number | undefined {
    return this.messages.shift();
  }

  wait(): Promise<number> {
    const msg = this.poll();
    if (msg !== undefined) {
      return Promise.resolve(msg);
    }
    const { promise, resolve } = Promise.withResolvers<number>();
    this.waiter = resolve;
    return promise;
  }
}

class Host implements exe.WasmHost {
  consoleDom = document.createElement("pre");
  consoleOutput = new ArrayBuffer(0, { maxByteLength: 10 << 10 });
  window_: HTMLCanvasElement | undefined;

  surfaces: Map<number, HTMLCanvasElement> = new Map();
  messageQueue = new MessageQueue();

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
      if (ret instanceof Promise) {
        ret.then((ret) => this.finishSync(msg.ret, ret));
        return;
      }
      this.finishSync(msg.ret, ret);
    }
  }

  finishSync(retPtr: number, ret: number): void {
    if (ret == 0) throw new Error();
    const ints = new Int32Array(this.wasmMemory.buffer, retPtr, 1);
    ints[0] = ret;
    Atomics.notify(ints, 0, 1);
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

    const id = 1;
    this.surfaces.set(id, surface);
    return id;
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

  render(window_id: number, surface_id: number) {
    const surface = this.surfaces.get(surface_id)!;
    this.window_!.getContext("2d")!.drawImage(surface, 0, 0);
  }

  set_pixels(id: number, ptr: number, len: number): void {
    const copy = new Uint8ClampedArray(
      this.wasmMemory.buffer,
      ptr,
      len,
    ).slice();
    const surface = this.surfaces.get(id)!;
    const imageData = new ImageData(copy, surface.width);
    surface.getContext("2d")!.putImageData(imageData, 0, 0);
  }

  poll_message(): number {
    return this.messageQueue.poll() ?? -1;
  }

  wait_message(): Promise<number> {
    return this.messageQueue.wait();
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
