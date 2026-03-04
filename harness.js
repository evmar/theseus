const fs = require("node:fs");

const utf8 = new TextDecoder();

async function main() {
  const buf = fs.readFileSync("target/wasm32-unknown-unknown/release/exe.wasm");
  const memory = new WebAssembly.Memory({
    initial: (0x40_0000 * 2) / (64 << 10),
  });
  const imports = {
    env: { memory },
    host: {
      console_log(addr, len) {
        let text = utf8.decode(memory.buffer.slice(addr, addr + len));
        console.log(text);
      },
      panic(addr, len) {
        let text = utf8.decode(memory.buffer.slice(addr, addr + len));
        throw new Error(text);
      },
    },
  };
  const mod = await WebAssembly.instantiate(buf, imports);
  const { entry_point } = mod.instance.exports;
  entry_point();
}

main().catch((err) => console.error(err));
