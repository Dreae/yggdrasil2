var WebpackDevServer = require("webpack-dev-server");
var webpack = require("webpack");

var compiler = webpack(require('./webpack.config.js'));
var devServer = new WebpackDevServer(compiler, {
    stats: {colors: true},
    contentBase: './static',
    historyApiFallback: true,
    proxy: {
      "/api/*": {
        target: "http://127.0.0.1:6767",
        secure: false
      }
    }
});

devServer.listen(8080, "localhost", function() {});
