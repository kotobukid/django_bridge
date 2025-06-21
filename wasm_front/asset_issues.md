# Trunk静的ファイル配信のトラブルシューティング

## 問題の概要

Leptos + Trunk開発環境で、PWAアイコンやmanifest.jsonなどの静的ファイルが正しく配信されない問題。

## 症状

- ブラウザコンソールで「Download error or resource isn't a valid image」エラー
- PWAアイコンが表示されない
- manifest.jsonは正常に配信される
- 一部のアイコンファイルは404エラー（Page not found）

## 根本原因

**Trunkの`rel="copy-file"`と`rel="copy-dir"`の動作の違い**

### `rel="copy-dir"`の場合
```html
<link data-trunk rel="copy-dir" href="icons">
```
- `icons/`ディレクトリ内のファイルを**ルート直下に展開**
- `icons/apple-touch-icon.png` → `dist/apple-touch-icon.png`
- アクセスURL: `http://localhost:8080/apple-touch-icon.png`

### `rel="copy-file"`の場合
```html
<link data-trunk rel="copy-file" href="icons/apple-touch-icon.png">
```
- ディレクトリ構造を保持してコピー
- `icons/apple-touch-icon.png` → `dist/icons/apple-touch-icon.png`
- アクセスURL: `http://localhost:8080/icons/apple-touch-icon.png`

## 診断方法

### 1. curlでファイルの存在確認
```bash
# manifest.jsonの確認
curl -I http://127.0.0.1:8080/manifest.json

# アイコンファイルの確認（ルート直下）
curl -I http://127.0.0.1:8080/apple-touch-icon.png

# アイコンファイルの確認（iconsディレクトリ内）
curl -I http://127.0.0.1:8080/icons/apple-touch-icon.png
```

### 2. レスポンスヘッダーの確認
- **正常**: `content-type: image/png`
- **エラー**: `content-type: text/html`（404ページ）

## 解決方法

### パターン1: ディレクトリ構造を保持する場合

**index.html:**
```html
<!-- ファイルを個別にコピー -->
<link data-trunk rel="copy-file" href="icons/apple-touch-icon.png">
<link data-trunk rel="copy-file" href="icons/favicon-32x32.png">
<!-- ... 他のファイル -->

<!-- HTML内参照 -->
<link rel="apple-touch-icon" href="icons/apple-touch-icon.png">
```

**manifest.json:**
```json
{
  "icons": [
    {
      "src": "icons/apple-touch-icon.png",
      "sizes": "180x180",
      "type": "image/png"
    }
  ]
}
```

### パターン2: ルート直下に展開する場合（推奨）

**index.html:**
```html
<!-- ディレクトリ全体をコピー（ルート直下に展開） -->
<link data-trunk rel="copy-file" href="icons/apple-touch-icon.png">
<link data-trunk rel="copy-file" href="icons/favicon-32x32.png">
<!-- または -->
<!-- 注意: copy-dirはルート直下に展開される -->

<!-- HTML内参照 -->
<link rel="apple-touch-icon" href="apple-touch-icon.png">
```

**manifest.json:**
```json
{
  "icons": [
    {
      "src": "apple-touch-icon.png",
      "sizes": "180x180", 
      "type": "image/png"
    }
  ]
}
```

## 重要なポイント

### 1. パス整合性の確保
- **Trunk設定**、**HTML参照**、**manifest.json参照**の3箇所でパスを統一する
- 一箇所でも不整合があるとPWAが正しく動作しない

### 2. copy-dirの動作理解
- `rel="copy-dir"`は直感的でない動作をする
- ディレクトリ構造を保持**しない**
- 個別ファイル指定の方が予測可能

### 3. 開発サーバーでの確認
- ブラウザキャッシュをクリアして確認
- curlで実際のHTTPレスポンスを確認
- デベロッパーツールのNetworkタブでHTTPステータスを確認

## 避けるべきパターン

### ❌ 混在パターン
```html
<!-- これは動作しない -->
<link data-trunk rel="copy-dir" href="icons">
<link rel="apple-touch-icon" href="icons/apple-touch-icon.png">
```
→ ファイルはルート直下に配置されるが、HTMLは`icons/`サブディレクトリを参照

### ❌ manifest.jsonとHTMLの不整合
```json
// manifest.json
"src": "icons/apple-touch-icon.png"
```
```html
<!-- index.html -->
<link rel="apple-touch-icon" href="apple-touch-icon.png">
```
→ パスが一致しない

## トラブルシューティング手順

1. **ファイル存在確認**: curlでアクセス可能か確認
2. **パス整合性確認**: HTML、manifest.json、実際のファイル配置を比較
3. **Trunk設定確認**: `rel="copy-file"`と`rel="copy-dir"`の使い分け
4. **キャッシュクリア**: ブラウザの強制リロード
5. **開発サーバー再起動**: Trunkサーバーの再起動

## 参考

- [Trunk Asset Handling](https://trunkrs.dev/assets/)
- [PWA Manifest Specification](https://web.dev/add-manifest/)

## 関連する非推奨警告の修正

Chrome等で表示される警告の対応：

```html
<!-- 非推奨 -->
<meta name="apple-mobile-web-app-capable" content="yes">

<!-- 推奨（両方記述） -->
<meta name="mobile-web-app-capable" content="yes">
<meta name="apple-mobile-web-app-capable" content="yes">
```