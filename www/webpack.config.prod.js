const { merge } = require('webpack-merge');
const common = require('./webpack.config.js');

module.exports = merge(common, {
    mode: 'production',
    performance: {
        hints: false,
    },
    optimization: {
        mangleWasmImports: false,
    },
    devtool: false
});
