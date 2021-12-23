const path = require("path");
const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

const FRONTEND_DIR = "frontend";
const IS_DEVELOPMENT = process.env.NODE_ENV !== "production";

// During development route /api requests to this URL
const DEV_PROXY = process.env.DEV_PROXY || "http://localhost:8000";

let localCanisters, prodCanisters, canisters;

function initCanisterIds() {
  try {
    localCanisters = require(path.resolve(
      ".dfx",
      "local",
      "canister_ids.json"
    ));
  } catch (error) {
    console.log("No local canister_ids.json found. Continuing production");
  }
  try {
    prodCanisters = require(path.resolve("canister_ids.json"));
  } catch (error) {
    console.log("No production canister_ids.json found. Continuing with local");
  }

  const network =
    process.env.DFX_NETWORK ||
    (process.env.NODE_ENV === "production" ? "ic" : "local");

  canisters = network === "local" ? localCanisters : prodCanisters;

  for (const canister in canisters) {
    process.env[canister.toUpperCase() + "_CANISTER_ID"] =
      canisters[canister][network];
  }
}
initCanisterIds();

module.exports = {
  target: "web",
  mode: IS_DEVELOPMENT ? "development" : "production",
  entry: path.join(__dirname, "src", FRONTEND_DIR, "src", "index.ts"),
  devtool: IS_DEVELOPMENT ? "source-map" : false,
  optimization: {
    minimize: !IS_DEVELOPMENT,
    minimizer: [new TerserPlugin()],
  },
  resolve: {
    extensions: [".js", ".ts", ".jsx", ".tsx"],
  },
  output: {
    filename: "[id].bundle.js",
    path: path.join(__dirname, "dist", FRONTEND_DIR),
  },
  module: {
    rules: [
      { test: /\.(ts|tsx|jsx)$/, loader: "ts-loader" },
      { test: /\.js$/, loader: "source-map-loader" },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(__dirname, "src", FRONTEND_DIR, "src", "index.html"),
      cache: false,
    }),
    new CopyPlugin({
      patterns: [
        {
          from: path.join(__dirname, "src", FRONTEND_DIR, "assets"),
          to: path.join(__dirname, "dist", FRONTEND_DIR),
        },
      ],
    }),
  ],
  // proxy /api to port 8000 during development
  devServer: {
    proxy: {
      "/api": {
        target: DEV_PROXY,
        // changeOrigin: true,
        // pathRewrite: {
        //   "^/api": "/api",
        // },
      },
    },
    static: path.join(__dirname, "src", FRONTEND_DIR, "assets"),
  },
};
