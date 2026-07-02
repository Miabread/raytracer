const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

console.log(__dirname);

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: 'index.html',
        }),
        new WasmPackPlugin({
            crateDirectory: __dirname,
        }),
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true,
    },
    watchOptions: {
        aggregateTimeout: 200,
        poll: 200,
    },
};
