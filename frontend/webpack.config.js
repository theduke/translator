const path = require('path');

const config = {
  context: path.resolve(__dirname, 'src'),
  devtool: 'eval-source-map',

  entry: {
    bundle: [
      'babel-polyfill',
      path.resolve(__dirname, 'src/app'),
    ],
  },

  output: {
    path: path.resolve(__dirname, 'dist/assets'),
    filename: '[name].js',
    publicPath: '/assets/'
  },

  resolve: {
    extensions: ['.tsx', '.ts', '.jsx', '.js'],
    alias: {
      assets: path.resolve(__dirname, 'assets'),
      styles: path.resolve(__dirname, 'styles'),
      translator: path.resolve(__dirname, 'src'),
    },
  },

  module: {
    rules: [
      {
        test: /\.tsx?$/,
        enforce: 'pre',
        loader: 'tslint-loader',
        options: {
          typeCheck: false,
        }
      },
      {
        test: /\.tsx?$/,
        loaders: [
          'babel-loader',
          'awesome-typescript-loader',
        ],
      },
      {
        test: /\.jsx?$/,
        loaders: [
          'babel-loader',
        ],
      },
      {
        test: /\.css$/,
        loaders: ['style-loader', 'css-loader'],
      },
      {
        test: /\.scss$/,
        loaders: [
          'style-loader',
          'css-loader',
          {
            loader: 'sass-loader',
            options: {
              includePaths: ["styles"],
            },
          },
        ],
      },
      // Images.
      {
        test: /\.(jpe?g|png|gif|svg|ttf|eot)(\?v=[0-9]\.[0-9]\.[0-9])?$/i,
        use: [
          {
            loader: 'file-loader',
            options: {
              name: '[name]_[hash].[ext]',
              publicPath: '/assets/',
              outputPath: 'img/',
            },
          },
        ],
      },
      {
        test: /\.woff(2)?(\?v=[0-9]\.[0-9]\.[0-9])?$/,
        use: [
          {
            loader: 'url-loader',
            options: {
              limit: 10000,
              mimetype: 'application/font-woff',
            },
          },
        ],
      },
    ],
  },

  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    port: 9000,
    historyApiFallback: true,
    hot: true,
  },

};
module.exports = config;
