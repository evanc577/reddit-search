import resolve from "rollup-plugin-node-resolve";
import { terser } from "rollup-plugin-terser"

export default {
    input: "script/date_picker",
    output: {
        file: "build/bundle.js",
        format: "iife",
        plugins: [
            terser()
        ]
    },
    plugins: [
        resolve(),
    ]
};
