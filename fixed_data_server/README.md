# Fixed Data Server

WIXOSS Trading Card Game データベースのフィーチャーオーバーライド管理とadmin_backend同期機能を提供するHTTPサーバーです。

## 概要

- **フィーチャーオーバーライド管理**: カードの特徴（フィーチャー）を手動で修正・管理
- **admin_backend同期**: 複数の開発環境間でデータを同期
- **Web UI連携**: フロントエンド（wasm_front）からの編集をサポート

## 起動方法

### 1. 環境変数設定

`.env`ファイルに以下を設定：

```bash
# データベース接続
DATABASE_URL=postgres://user:password@localhost:5432/wx_db

# admin_backend同期設定（オプション）
ADMIN_BACKEND_URL=https://your-admin-backend.com:50051
ADMIN_API_KEY=your-api-key
```

### 2. サーバー起動

```bash
# 開発モード
cargo run

# デバッグログ付き
RUST_LOG=debug cargo run

# プロダクションビルド
cargo build --release
./target/release/fixed_data_server
```

サーバーは `http://127.0.0.1:8004` で起動します。

## API エンドポイント

### フィーチャーオーバーライド管理

| エンドポイント | メソッド | 説明 |
|---|---|---|
| `/api/overrides` | GET | 全オーバーライド一覧取得 |
| `/api/overrides` | POST | 新規オーバーライド作成 |
| `/api/overrides/{pronunciation}` | GET | 特定の読み方のオーバーライド取得 |
| `/api/overrides/{pronunciation}` | PUT | 特定の読み方のオーバーライド更新 |
| `/api/overrides/{pronunciation}` | DELETE | 特定の読み方のオーバーライド削除 |

### admin_backend同期

| エンドポイント | メソッド | 説明 |
|---|---|---|
| `/api/sync/status` | GET | 同期ステータス確認 |
| `/api/sync/push` | POST | ローカルデータをadmin_backendにプッシュ |
| `/api/sync/pull` | POST | admin_backendからデータをプル |
| `/api/sync/bidirectional` | POST | 双方向同期（プッシュ→プル） |

### その他

| エンドポイント | メソッド | 説明 |
|---|---|---|
| `/api/check-consistency` | GET | データ整合性チェック |
| `/api/export` | GET | 全データエクスポート |
| `/api/import` | POST | データインポート |
| `/health` | GET | ヘルスチェック |

## admin_backend同期の使用方法

### 1. 同期ステータス確認

```bash
curl http://localhost:8004/api/sync/status
```

admin_backendとの接続状況とローカルデータの件数を確認できます。

### 2. ローカルデータをadmin_backendにプッシュ

```bash
curl -X POST http://localhost:8004/api/sync/push
```

ローカルのPostgreSQLからfeature overrideデータを読み取り、admin_backendのSQLiteに送信します。

### 3. admin_backendからデータをプル

```bash
curl -X POST http://localhost:8004/api/sync/pull
```

admin_backendからfeature overrideデータを取得し、ローカルのPostgreSQLに反映します。

### 4. 双方向同期

```bash
curl -X POST http://localhost:8004/api/sync/bidirectional
```

プッシュとプルを順番に実行し、双方向でデータを同期します。

## フィーチャーオーバーライドの使用方法

### 1. Web UIからの編集

1. `trunk serve`でフロントエンドを起動
2. ブラウザで `http://localhost:8080/edit/{pronunciation}` にアクセス
3. カードの特徴を編集して保存

### 2. APIからの直接操作

#### 新規作成・更新

```bash
curl -X POST http://localhost:8004/api/overrides \
  -H "Content-Type: application/json" \
  -d '{
    "pronunciation": "エンケンノマイ",
    "features": ["バニッシュ", "エクシード", "ライフクラッシュ"],
    "burst_features": [],
    "note": "手動修正"
  }'
```

#### 取得

```bash
curl http://localhost:8004/api/overrides/エンケンノマイ
```

#### 削除

```bash
curl -X DELETE http://localhost:8004/api/overrides/エンケンノマイ
```

## 対応フィーチャー

### カードフィーチャー

以下の特徴に対応（`shared/feature/src/feature.rs`に基づく）：

- **攻撃系**: ダブルクラッシュ、バニッシュ、ダメージ、Sランサー、アサシン等
- **妨害系**: ハンデス、凍結、課税、デッキ落下等  
- **防御系**: ガード、バニッシュ耐性、ダウン、アタック無効等
- **資源系**: ドロー、エクシード、リコレクト、エナチャージ等
- **固有系**: チャーム、クラフト、アクセ、ライズ等

### バーストフィーチャー

- LB1点防御、LB2点防御、LBガード回収等

## データベーススキーマ

```sql
-- PostgreSQL (ローカル)
CREATE TABLE wix_card_feature_override (
    pronunciation TEXT PRIMARY KEY,
    fixed_bits1 BIGINT NOT NULL,
    fixed_bits2 BIGINT NOT NULL, 
    fixed_burst_bits BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    note TEXT NOT NULL DEFAULT ''
);
```

## トラブルシューティング

### 1. データベース接続エラー

```bash
# 環境変数確認
echo $DATABASE_URL

# PostgreSQL接続テスト
psql $DATABASE_URL -c "SELECT 1;"
```

### 2. admin_backend接続エラー

```bash
# 環境変数確認
echo $ADMIN_BACKEND_URL
echo $ADMIN_API_KEY

# ステータス確認
curl http://localhost:8004/api/sync/status
```

### 3. フィーチャー変換エラー

ログで`Unknown feature:`警告が出る場合、`src/handlers/overrides.rs`の変換マッピングを確認してください。

### 4. CORS エラー

フロントエンドからのアクセスでCORSエラーが発生する場合、サーバーは既に`CorsLayer::new().allow_origin(Any)`で設定済みです。

## 開発・デバッグ

### ログレベル設定

```bash
# 詳細ログ
RUST_LOG=debug cargo run

# 特定モジュールのみ
RUST_LOG=fixed_data_server::handlers::sync=debug cargo run
```

### テスト実行

```bash
# 単体テスト
cargo test

# 統合テスト
./test_override.sh
```

## アーキテクチャ

```
[Web UI] ←→ [fixed_data_server] ←→ [PostgreSQL]
              ↕ (gRPC/TLS)
         [admin_backend] ←→ [SQLite]
```

- **fixed_data_server**: HTTPサーバー（本プロジェクト）
- **admin_backend**: gRPCサーバー（同期ハブ）
- **PostgreSQL**: ローカルデータベース
- **SQLite**: admin_backendの集約データベース

## 関連プロジェクト

- `../admin_backend`: gRPC同期サーバー
- `../wasm_front`: フロントエンドUI
- `../shared/feature`: 特徴定義