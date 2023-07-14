import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import * as path from "path";

export default defineConfig({
    base: "/static/",

    plugins: [solid()],
    server: {
        port: 3000,
    },
    build: {
        target: "esnext",
    },

    resolve: {
        alias: {
            src: path.resolve("src/"),
        },
    },
    css: {
        modules: {
            localsConvention: "camelCase",
        },
    },
});
