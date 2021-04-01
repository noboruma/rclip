require('file-loader?name=[name].[ext]!./index.html');
require('file-loader?name=[name].[ext]!./cli.html');
require('file-loader?name=[name].[ext]!./success.html');
require('file-loader?name=[name].[ext]!./cancel.html');
require('file-loader?name=[name].[ext]!./footer.html');
require('file-loader?name=[name].[ext]!./styles.css');
require('file-loader?name=[name].[ext]!./utils.js');
require('file-loader?name=[name].[ext]!./favicon.ico');
require('file-loader?name=[name].[ext]!./sitemap.xml');
module.exports = {
    handleGenerate: function(callback) {
        import("../pkg").then(lib => {
            lib.open(callback);
        });
    },
    handleLogin: function(email, passwd, callback) {
        import("../pkg").then(lib => {
            lib.login(email, passwd, callback);
        });
    },
    handleCopy: function(token, namespace, data, callback) {
        import("../pkg").then(lib => {
            lib.copy(token, namespace, data, callback);
        });
    },
    handlePaste: function(token, namespace, callback) {
        import("../pkg").then(lib => {
            lib.paste(token, namespace, callback);
        });
    }
};
