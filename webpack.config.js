var webpack = require('webpack');

module.exports = {
  entry: {
    app: ["./assets/index.js"]
  },
  output: {
    path: __dirname + "/static",
    filename: "bundle.js"
  },
  devtool: "eval",
  debug: true,
  plugins: [
    new webpack.ProvidePlugin({
      riot: "riot"
    })
  ],
  module: {
    preLoaders: [
      { test: /\.tag$/, exclude: /node_modules/, loader: 'riotjs-loader', query: { type: 'babel' } }
    ],
    loaders: [
      { test: /index\.tpl/, loader: 'file?name=[name].[ext]' },
      { test: /\.css$/, loaders: ["style", "css"] },
      { test: /\.js$|\.tag$/, exclude: /node_modules/, loader: 'babel', query: { presets: ["es2015"] } }
    ]
  }
}
