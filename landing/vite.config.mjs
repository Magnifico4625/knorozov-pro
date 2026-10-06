import { defineConfig } from "vite";
import { fileURLToPath } from "node:url";

export default defineConfig({
  root: fileURLToPath(new URL(".", import.meta.url)),
  base: "/knorozov-pro/",
  server: { host: "0.0.0.0", port: 4173, strictPort: true, allowedHosts: ["terminal.local"] },
  preview: { host: "0.0.0.0", port: 4173, strictPort: true, allowedHosts: ["terminal.local"] },
  build: { target: "es2021", outDir: "dist", emptyOutDir: true },
});
