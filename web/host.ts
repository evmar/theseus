import * as exe from "./winapi_wasm.js";

function init(memory: WebAssembly.Memory) {
  if (!window.SharedArrayBuffer) {
    alert("SharedArrayBuffer is not supported");
  }

  const consoleDom = document.createElement("pre");
  consoleDom.id = "console";
  document.body.appendChild(consoleDom);

  const consoleOutput = new ArrayBuffer(0, { maxByteLength: 10 << 10 });
  window.onmessage = (e) => {
    const buffer = memory.buffer;
    const [msg] = e.data;
    switch (msg) {
      case "wake":
        const [, ptr, len, done] = e.data;
        const inBuf = new Uint8Array(buffer, ptr, len);
        const ofs = consoleOutput.byteLength;
        consoleOutput.resize(ofs + len);
        const outBuf = new Uint8Array(consoleOutput, ofs, len);
        outBuf.set(inBuf);
        consoleDom.innerText = new TextDecoder().decode(consoleOutput);
        break;
    }
    console.log(msg);
  };
}

async function main() {
  const memory = new WebAssembly.Memory({
    initial: 32, // in units of 64KB pages, 2mb
    maximum: 1024, // 64mb
    shared: true,
  });
  init(memory);
  await exe.default(/* module */ undefined, memory);
  exe.main();
}
main().catch((e) => console.error(e));
