build:
    trunk build --release --public-url rgrep
    for file in dist/*.wasm; do wasm-opt -Oz -o $file $file; done

serve:
    trunk serve
