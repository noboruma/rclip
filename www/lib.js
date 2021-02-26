require('file-loader?name=[name].[ext]!./index.html');
require('file-loader?name=[name].[ext]!./cli.html');
require('file-loader?name=[name].[ext]!./styles.css');
require('file-loader?name=[name].[ext]!./utils.js');
module.exports = {
    handleGenerate: function(callback) {
        import("../pkg").then(lib => {
            lib.open(callback);
        });
    },
    handleCopy: function(token, data, callback) {
        import("../pkg").then(lib => {
            lib.copy(token, data, callback);
        });
    },
    handlePaste: function(token, callback) {
        import("../pkg").then(lib => {
            lib.paste(token, callback);
        });
    }
};
