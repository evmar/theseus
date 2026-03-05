const fs = require("node:fs");

const utf8 = new TextDecoder();

async function main() {
  const args = process.argv.slice(2);
  if (args.length !== 1) {
    throw new Error("Usage: node harness.js <path-to-wasm>");
  }
  const [path] = args;
  const buf = fs.readFileSync(path);
  const memory = new WebAssembly.Memory({
    initial: (0x40_0000 * 2) / (64 << 10),
  });
  const imports = {
    env: { memory },
    host: {
      print(addr, len) {
        let text = utf8.decode(memory.buffer.slice(addr, addr + len));
        process.stdout.write(text);
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
