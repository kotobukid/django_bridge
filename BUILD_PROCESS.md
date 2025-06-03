# Django Bridge ビルド・デプロイ完全手順

## 前提条件
- PostgreSQLが起動していること
- `.env`ファイルが設定済みであること
- Rust、Node.js、Pythonがインストールされていること
- `wasm-pack`がインストールされていること（`cargo install wasm-pack`）
- `cargo-make`がインストールされていること（`cargo install cargo-make`）

## ビルド手順

### 1. データベースのセットアップ（初回のみ）
```bash
# Djangoマイグレーションの作成
cargo make manage makemigrations

# マイグレーションの適用
cargo make manage migrate

# DjangoモデルをRust構造体に同期
cargo make syncdb
```

### 2. カードデータのスクレイピング（必要に応じて）
```bash
# 特定の製品のカードデータをスクレイピング
cd shared/webapp
cargo run --example scrape WXDi-P00  # 製品コードを指定

# 単一カードのスクレイピング
cargo run --example scrape_single WXDi-P00-001  # カードコードを指定
```

### 3. 静的データの生成
```bash
# データベースからRustコードを生成（datapack/src/gen/cards.rs）
cargo make static
```

### 4. WASMモジュールのビルド
```bash
# Linux/macOSの場合
cargo make wasm_linux

# Windowsの場合
cargo make wasm

# これにより以下のファイルが生成・配置されます：
# - front/public/pkg/datapack_bg.wasm
# - front/static/pkg/datapack.js
# - front/static/pkg/datapack.d.ts
# - front/static/pkg/datapack_bg.wasm.d.ts
```

### 5. フロントエンドのビルド
```bash
# 依存関係のインストール（初回のみ）
cd front
npm install

# 開発サーバーの起動
cd /home/kakehashi/RustroverProjects/django_bridge
cargo make front_server  # ポート3001で起動

# または本番ビルド
cargo make generate
```

### 6. バックエンドサーバーの起動
```bash
# Rust APIサーバー（ポート8002）
cargo make server

# Django管理画面（ポート8003）- 必要に応じて
cargo make manage runserver 0.0.0.0:8003
```

## トラブルシューティング

### WASMロードエラー（Unexpected...）の場合

1. **WASMファイルの確認**
```bash
# WASMファイルが正しい場所にあるか確認
ls -la front/public/pkg/datapack_bg.wasm
ls -la front/static/pkg/datapack.js
```

2. **再ビルド**
```bash
# クリーンビルド
cd datapack
cargo clean
cd ..
cargo make wasm_linux
```

3. **ブラウザキャッシュのクリア**
- 開発者ツールを開いてハード再読み込み（Ctrl+Shift+R / Cmd+Shift+R）

### fetch_by_f_bitsエラーの場合

1. **関数シグネチャの確認**
```bash
# 生成されたTypeScript定義を確認
cat front/static/pkg/datapack.d.ts | grep fetch_by_f_bits
```

2. **BigInt使用の確認**
```javascript
// 正しい使用方法
fetch_by_f_bits(BigInt("0"), BigInt("0"))

// 間違い（数値を直接渡す）
fetch_by_f_bits(0, 0)  // エラー
```

## 完全なビルドフロー（クリーンビルド）

```bash
# 1. データベース更新
cargo make manage migrate
cargo make syncdb

# 2. カードデータ更新（必要に応じて）
cd shared/webapp
cargo run --example scrape WXDi-P00
cd ../..

# 3. 静的データ生成
cargo make static

# 4. WASMビルド
cargo make wasm_linux

# 5. フロントエンドビルド
cargo make generate

# 6. サーバー起動
cargo make server  # 別ターミナルで
cargo make front_server  # 別ターミナルで
```

## 環境変数

`.env`ファイルに以下が必要：
```
DB_HOST=localhost
DB_PORT=5432
DB_USER=your_user
DB_PASSWORD=your_password
DB_NAME=your_database
```

## ポート一覧
- 3001: Nuxt.js開発サーバー
- 8002: Rust APIサーバー
- 8003: Django管理画面

## 注意事項
- `datapack/src/gen/`ディレクトリは自動生成されるため、手動編集しない
- WASMビルド後は必ずブラウザのキャッシュをクリアする
- `cargo make static`は必ずカードデータ変更後に実行する