const fs = require("node:fs");

const utf8 = new TextDecoder();

async function main() {
  const buf = fs.readFileSync("t.wasm");
  const memory = new WebAssembly.Memory({ initial: 1000 });
  const imports = {
    env: { memory },
    host: {
      console_log(addr, len) {
        console.log("console.log", addr.toString(16), len);
        let text = utf8.decode(memory.buffer.slice(addr, addr + len));
        console.log(JSON.stringify(text));
      },
      panic(text, len) {
        let text2 = utf8.decode(memory.buffer.slice(addr, addr + len));
        console.error("panic", text2);
      },
    },
  };
  const mod = await WebAssembly.instantiate(buf, imports);
  const { x401000 } = mod.instance.exports;
  x401000();
}

main().catch((err) => console.error(err));
