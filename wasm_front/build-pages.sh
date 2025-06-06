#!/bin/bash
set -e

# Trunkビルド実行
trunk build --config Trunk-pages.toml --release --minify

# 追加ファイル作成
touch ../dist-pages/.nojekyll
cp ../gh-pages-404.html ../dist-pages/404.html

echo "GitHub Pages用ビルド完了"
