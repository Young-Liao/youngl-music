import js from "@eslint/js";
import globals from "globals";
import ts_eslint from "typescript-eslint";
import pluginVue from "eslint-plugin-vue";
import {defineConfig} from "eslint/config";

export default defineConfig([
    {
        ignores: [
            "**/vite-env.d.ts"
        ]
    },
    {
        files: ["**/*.{js,mjs,cjs,ts,mts,cts,vue}"],
        plugins: {js},
        extends: ["js/recommended"],
        languageOptions: {globals: globals.browser},
    },
    ts_eslint.configs.recommended,
    pluginVue.configs["flat/essential"],
    {files: ["**/*.vue"], languageOptions: {parserOptions: {parser: ts_eslint.parser}}},
]);
