import { resolve } from "node:path";
import { defineConfig } from "vite";

export default defineConfig({
  base: "./",
  build: {
    rolldownOptions: {
      input: {
        main: resolve(import.meta.dirname, "index.html"),
        sbtalker: resolve(import.meta.dirname, "sbtalker/index.html"),
      },
    },
  },
});
