build:
    trunk build --release --public-url rgrep
    for file in dist/*.wasm; do wasm-opt -Oz -o $file $file; done

serve:
    trunk serve

push:
    just build
    git checkout gh-pages
    rm *.png *.svg *.ico *.wasm *.js *.html
    git add dist/*
    git mv --force dist/* .
    git add -u
    git commit -m "Update gh-pages"
    git push origin HEAD
    git checkout -
