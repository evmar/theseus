const fs = require("node:fs");

const utf8 = new TextDecoder();

async function main() {
  const buf = fs.readFileSync(
    "target/wasm32-unknown-unknown/release/wasm.wasm",
  );
  const memory = new WebAssembly.Memory({
    initial: (0x40_0000 * 2) / (64 << 10),
  });
  const imports = {
    env: { memory },
    host: {
      console_log(addr, len) {
        console.log("console.log", addr.toString(16), len);
        let text = utf8.decode(memory.buffer.slice(addr, addr + len));
        console.log(text);
      },
      panic(addr, len) {
        console.log("panic", addr.toString(16), len);
        let text = utf8.decode(memory.buffer.slice(addr, addr + len));
        console.error("panic", JSON.stringify(text));
      },
    },
  };
  const mod = await WebAssembly.instantiate(buf, imports);
  const { x401000 } = mod.instance.exports;
  x401000();
}

main().catch((err) => console.error(err));
