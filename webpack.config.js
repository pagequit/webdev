// eslint-disable-next-line @typescript-eslint/no-var-requires
const path = require("path");

module.exports = {
    mode: "development",
    entry: "./index.ts",
    output: {
        path: path.resolve(__dirname, "public/dist"),
        filename: "index.js",
        clean: true,
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: "ts-loader",
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: [".ts", ".js", ".json", ".wasm"],
    },
};
