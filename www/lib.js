require('file-loader?name=[name].[ext]!./index.html');
module.exports = {
    handleCopy: function(token, data) {
        import("../pkg").then(lib => {
            lib.copy(token, data);
        });
    },
    handlePaste: function(token, callback) {
        import("../pkg").then(lib => {
            lib.paste(token, callback);
        });
    }
};
