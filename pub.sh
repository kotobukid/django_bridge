#!/bin/sh
# GithubPagesへ公開するスクリプト

# まずビルド
cargo make trunk_pages

# gh-pages仕様への対応
touch ./dist-pages/.nojekyll
cp ./wasm_front/gh-pages-404.html ./dist-pages/404.html
cp ./wasm_front/CNAME ./dist-pages/CNAME

# 一時的なブランチを作成してチェックアウト
git checkout -b temp-deploy

# ビルドファイルの追加とコミット
git add dist-pages/ --force
git commit -m "Update dist files with relative paths for GitHub Pages"

# gh-pagesブランチへのデプロイ
DEPLOY_HASH=$(git subtree split --prefix dist-pages temp-deploy)
git push github $DEPLOY_HASH:gh-pages --force

# 元のブランチに戻る
git checkout -
# 一時ブランチの削除
git branch -D temp-deploy