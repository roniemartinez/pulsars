import globals from "globals";
import pluginJs from "@eslint/js";
import pluginReact from "eslint-plugin-react";

/** @type {import('eslint').Linter.Config[]} */
export default [
  { files: ["**/*.{js,mjs,cjs,jsx}"] },
  { ignores: ["src-tauri/gen/*", "src-tauri/target/*"] },
  { languageOptions: { globals: globals.browser } },
  pluginJs.configs.recommended,
  pluginReact.configs.flat.recommended,
  { rules: { "react/react-in-jsx-scope": "off" } },
  { settings: { react: { version: "detect" } } },
];
