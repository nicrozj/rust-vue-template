import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import vue from "@vitejs/plugin-vue";
import path from "path";

console.log("Vite config loaded:", { host: "0.0.0.0", port: 5173 });

export default defineConfig({
	server: {
		host: "0.0.0.0",
		port: 5173,
	},
	plugins: [vue(), tailwindcss()],
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "./src"),
		},
	},
});
