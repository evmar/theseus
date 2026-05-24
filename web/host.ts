import * as exe from "./mine_wasm.js";

class MessageQueue {
  private messages: Event[] = [];
  private waiter: ((value: Event) => void) | undefined;

  poll(): Event | undefined {
    return this.messages.shift();
  }

  wait(): Promise<Event> {
    const msg = this.poll();
    if (msg !== undefined) {
      return Promise.resolve(msg);
    }
    const { promise, resolve } = Promise.withResolvers<Event>();
    this.waiter = resolve;
    return promise;
  }

  private enqueue = (e: Event) => {
    e.preventDefault();
    if (this.waiter) {
      this.waiter(e);
      this.waiter = undefined;
    } else {
      this.messages.push(e);
    }
  };
  private discard = (e: Event) => {
    e.preventDefault();
  };

  listen(dom: HTMLCanvasElement) {
    dom.onmousedown = this.enqueue;
    dom.onmouseup = this.enqueue;
    dom.onmousemove = this.enqueue;
    dom.oncontextmenu = this.discard;
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
    const ret = (this as any)[msg.func](...msg.args);
    if (msg.retAddr) {
      if (ret instanceof Promise) {
        ret.then((ret) => this.finishSync(msg.retAddr, ret));
        return;
      }
      this.finishSync(msg.retAddr, ret);
    }
  }

  finishSync(retAddr: number, ret: number | number[]): void {
    if (ret == 0) throw new Error();
    const arr = Array.isArray(ret) ? ret : [ret];
    const ints = new Int32Array(this.wasmMemory.buffer, retAddr, arr.length);
    ints.set(arr);
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
    this.messageQueue.listen(this.window_);
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

  private serializeMessage(event: Event): number[] {
    // see wasm.rs:parse_message
    const typeToCode: Record<string, number> = {
      mousedown: 2,
      mouseup: 3,
      mousemove: 4,
    };
    const code = typeToCode[event.type];
    if (code === undefined) throw new Error();
    switch (event.type) {
      case "mousedown":
      case "mouseup":
      case "mousemove": {
        const e = event as MouseEvent;
        return [typeToCode[e.type]!, e.offsetX, e.offsetY, 1 << e.button];
      }
      default:
        throw new Error();
    }
  }

  poll_message(): number[] {
    const event = this.messageQueue.poll();
    return event ? this.serializeMessage(event) : [-1];
  }

  async wait_message(): Promise<number[]> {
    const event = await this.messageQueue.wait();
    return this.serializeMessage(event);
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
