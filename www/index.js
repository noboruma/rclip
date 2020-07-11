require('file-loader?name=[name].[ext]!./index.html');
const js = import("./node_modules/remote-clipboard/remote_clipboard.js");
js.then(js => {
    js.copy('e4ae7e4c03394cf88aba63fb53665c59', 'from_wasm');
});
