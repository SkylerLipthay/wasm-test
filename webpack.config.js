const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  devServer: {
    compress: true,
    disableHostCheck: true,
    historyApiFallback: {
      disableDotRule: true
    },
    hot: false,
    inline: false
  },
  entry: './static/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './static/index.html',
      title: 'WASM Test'
    })
  ],
  mode: 'development'
};
