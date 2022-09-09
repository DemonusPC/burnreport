import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    assetsDir: "static",
  },
  define: {
    "process.env": process.env,
  },
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:8080",
      },
    },
  },
});
