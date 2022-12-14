import nodeResolve from "@rollup/plugin-node-resolve";
import terser from "@rollup/plugin-terser"

export default {
    input: "script/date_picker.js",
    output: {
        file: "build/bundle.js",
        format: "iife",
        plugins: [
            terser()
        ]
    },
    plugins: [
        nodeResolve(),
    ]
};
