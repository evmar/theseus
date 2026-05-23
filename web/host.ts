import * as exe from "./mine_wasm.js";

function init(worker: Worker, memory: WebAssembly.Memory) {
  if (!window.SharedArrayBuffer) {
    alert("SharedArrayBuffer is not supported");
  }

  const consoleDom = document.createElement("pre");
  consoleDom.id = "console";
  document.body.appendChild(consoleDom);

  let window_: HTMLCanvasElement;
  const consoleOutput = new ArrayBuffer(0, { maxByteLength: 10 << 10 });
  worker.onmessage = (e: MessageEvent<exe.Msg>) => {
    const buffer = memory.buffer;
    const msg = e.data;
    console.log("msg", msg);
    switch (msg[0]) {
      case "console_write": {
        const [, ptr, len, done] = msg;
        const inBuf = new Uint8Array(buffer, ptr, len);
        const ofs = consoleOutput.byteLength;
        consoleOutput.resize(ofs + len);
        const outBuf = new Uint8Array(consoleOutput, ofs, len);
        outBuf.set(inBuf);
        consoleDom.innerText = new TextDecoder().decode(consoleOutput);
        break;
      }

      case "create_surface": {
        const [, width, height, done] = msg;
        const surface = document.createElement("canvas");
        surface.width = width;
        surface.height = height;
        document.body.appendChild(surface);
        const ta = new Int32Array(buffer);
        console.log("writing atomic");
        Atomics.store(ta, done, 1);
        break;
      }

      case "create_window": {
        const [, title, width, height, done] = msg;
        window_ = document.createElement("canvas");
        window_.className = "window";
        window_.width = width;
        window_.height = height;
        document.body.appendChild(window_);
        console.log("done is at", done.toString(16));
        const ta = new Int32Array(buffer, done, 1);
        console.log("writing atomic, prev is", ta[0]);
        ta[0] = 1;
        Atomics.notify(ta, 0, 1);
        console.log("notified atomic");
        break;
      }

      case "resize_window": {
        const [, id, width, height] = msg;
        window_.width = width;
        window_.height = height;
        break;
      }

      default:
        throw new Error(`unknown message: ${msg[0]}`);
    }
  };
}

async function main() {
  const memory = new WebAssembly.Memory({
    initial: 32, // in units of 64KB pages, 2mb
    maximum: 1024, // 64mb
    shared: true,
  });

  const worker = new Worker("./worker.js", { type: "module" });
  init(worker, memory);
  worker.postMessage(memory);
}
main().catch((e) => console.error(e));
