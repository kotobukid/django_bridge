# Admin Backend Sync - テスト手順

## 環境設定

### 1. 環境変数設定
```bash
export ADMIN_BACKEND_URL=https://ik1-341-30725.vs.sakura.ne.jp:50051
export ADMIN_BACKEND_API_KEY=ADM_1609d9734c364a1088b02f2f2b5bb154
export SYNC_CLIENT_ID=wx_db_local
```

### 2. データベース設定
PostgreSQLデータベースにwix_card_feature_overrideテーブルが存在することを確認。

## テスト手順

### 1. ビルド確認
```bash
cd /path/to/wx_db
cargo check -p fixed_data_server
cargo build --release -p fixed_data_server
```

### 2. サーバー起動
```bash
cargo run --release -p fixed_data_server
# または
cargo make fixed_data_server
```

### 3. 同期機能テスト

#### a. 接続確認
```bash
curl http://localhost:8004/api/sync/status | jq
```

#### b. プル同期（admin_backendから取得）
```bash
curl -X POST http://localhost:8004/api/sync/pull | jq
```

#### c. プッシュ同期（admin_backendへ送信）
```bash
curl -X POST http://localhost:8004/api/sync/push | jq
```

#### d. 双方向同期
```bash
curl -X POST http://localhost:8004/api/sync/bidirectional | jq
```

### 4. 既存機能確認
```bash
# 既存のオーバーライド機能も正常動作することを確認
curl http://localhost:8004/api/overrides | jq
curl http://localhost:8004/health
```

## 期待される結果

### 接続成功時
- `/api/sync/status`: admin_backend_connected: true
- server_time、total_feature_overrides等の情報表示

### 同期成功時
- items_affected: 数値
- success: true
- 適切なメッセージとdetails

### エラー時
- success: false
- 詳細なエラーメッセージ
- オフライン機能により既存機能は正常動作継続

## トラブルシューティング

### TLS接続エラー
- ADMIN_BACKEND_URLが正しく設定されているか確認
- admin_backendサーバーが起動しているか確認

### 認証エラー  
- ADMIN_BACKEND_API_KEYが正しく設定されているか確認
- APIキーが有効期限内か確認

### データベースエラー
- PostgreSQL接続情報が正しいか確認
- wix_card_feature_overrideテーブルが存在するか確認