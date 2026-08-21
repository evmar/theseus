// Note that the specific exe here doesn't matter, we just need the same types found in all of them.
import type * as exe from "./exe/basicdd/basicdd.js";
import type * as worker from "./worker.js";

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
  nextSurface = 1;
  messageQueue = new MessageQueue();

  constructor(public wasmMemory: WebAssembly.Memory) {
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
    const arr = Array.isArray(ret) ? ret : [ret];
    if (!Number.isFinite(arr[0]) || arr[0] == 0) {
      // For synchronization to work, we must put a non-zero value in the first slot.
      // If this hits we messed up the sync/non-sync ness of some API.
      throw new Error();
    }
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

    const id = this.nextSurface++;
    this.surfaces.set(id, surface);
    return id;
  }

  create_window(_title: string, width: number, height: number): number {
    this.window_ = document.createElement("canvas");
    this.window_.className = "window";
    this.window_.width = width;
    this.window_.height = height;
    document.body.appendChild(this.window_);
    this.messageQueue.listen(this.window_);
    return 1;
  }

  resize_window(_id: number, width: number, height: number): void {
    this.window_!.width = width;
    this.window_!.height = height;
  }

  render(_window_id: number, surface_id: number) {
    const surface = this.surfaces.get(surface_id)!;
    this.window_!.getContext("2d")!.drawImage(surface, 0, 0);
  }

  set_pixels(id: number, ptr: number, len: number): number {
    // TODO: investigate using OffscreenCanvas here instead,
    // https://news.ycombinator.com/item?id=48297805
    const copy = new Uint8ClampedArray(
      this.wasmMemory.buffer,
      ptr,
      len,
    ).slice();
    const surface = this.surfaces.get(id)!;
    const imageData = new ImageData(copy, surface.width);
    surface.getContext("2d")!.putImageData(imageData, 0, 0);
    return 1;
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

  write_file(_path: string, _ptr: number, _len: number): number {
    throw new Error('unimplemented');
  }
}

/// Programs to run, chosen with `?exe=`.
// TODO: we'll put program params in the values of this map.
const PROGRAMS = new Map([
  ["mine", {}],
  ["basicdd", {}],
  ["winapi", {}],
]);

async function main() {
  if (!window.SharedArrayBuffer) {
    document.body.innerText = "SharedArrayBuffer is not supported; possibly try reloading";
    return;
  }

  const wasmPageSize = 64 << 10;
  const memory = new WebAssembly.Memory({
    // memory args are in units of wasm page size
    initial: (2 << 20) / wasmPageSize,
    maximum: (128 << 20) / wasmPageSize,
    shared: true,
  });

  const host = new Host(memory);
  const worker = new Worker(new URL("./worker.js", import.meta.url), { type: "module" });
  worker.onmessage = (e) => host.onMessage(e);

  const params = new URLSearchParams(window.location.search);
  const name = params.get("exe") ?? "mine";
  const program = PROGRAMS.get(name);
  if (!program) {
    document.body.innerText = `no such program ${name}`;
    return;
  }

  const message: worker.StartMessage = {
    ...program,
    name,
    memory,
  };
  worker.postMessage(message);
}
main().catch((e) => console.error(e));
