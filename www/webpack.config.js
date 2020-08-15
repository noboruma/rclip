const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
module.exports = {
  entry: "./lib.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
    library: 'rclip',
    libraryTarget: 'var',
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, ".."),
    }),
  ],
};
