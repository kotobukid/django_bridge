# Django Bridge - Table Definition

このプロジェクトは、Axumベースのメインプロジェクト（`shared/webapp`）と連携して動作する特殊なDjangoプロジェクトです。

## 特殊な用途

通常のDjangoプロジェクトとは異なり、以下の限定的な目的で使用されます：

1. **モデル定義とマイグレーション**
   - WIXOSSカードデータベースのテーブル構造を定義
   - Djangoのマイグレーション機能を利用したスキーマ管理

2. **管理画面の提供**
   - Django Admin を使用した自動生成管理インターフェース
   - データの確認・編集・削除機能

3. **一時的なサーバー起動**
   - Axumプロジェクトからの要求に応じて動的に起動
   - 管理作業完了後は自動的に終了

## アーキテクチャ

```
Axumサーバー (shared/webapp)
       ↓ 管理画面が必要な時
Django開発サーバー (table_definition)
       ↓ 管理作業完了後
自動終了
```

## 主要コンポーネント

### 1. カスタム設定 (`table_definition/config.py`)
- Axumサーバーとの連携用設定
- CSRF信頼ホスト設定
- 管理画面専用アプリ構成

### 2. 動的URL設定 (`table_definition/urls.py`)
- カスタム管理画面ルート対応
- ヘルスチェックエンドポイント
- 静的ファイル配信

### 3. カスタム起動コマンド (`admin_server/management/commands/run_with_custom_admin.py`)
- 動的な管理画面URL設定
- CSRF信頼ポートの動的追加
- Axumサーバーからの設定注入

### 4. 管理画面カスタマイズ (`wix/admin.py`)
- WIXOSSカードデータ用の最適化された管理画面
- 検索・フィルタリング機能
- 関連データの効率的な表示

### 5. ヘルスチェック機能 (`admin_server/views.py`)
- Axumサーバーからの生存確認
- サーバー状態の監視

## 使用方法

### 基本的な起動
```bash
python manage.py run_with_custom_admin
```

### カスタムURL設定での起動
```bash
python manage.py run_with_custom_admin --admin-root "custom_admin/" --csrf-trust-port 8002
```

### マイグレーション
```bash
python manage.py makemigrations
python manage.py migrate
```

## 設定オプション

- `--admin-root`: 管理画面のカスタムURL root
- `--csrf-trust-port`: CSRF信頼ポート（Axumサーバー連携用）

## モデル構造

- **Card**: カード情報
- **Product**: プロダクト情報
- **CardType**: カードタイプ
- **Color**: カラー情報
- **Lrig**: LRIG情報
- **Klass**: クラス情報
- **Feature**: 特徴情報
- **Timing**: タイミング情報

## 注意事項

1. **本番環境での使用禁止**: このプロジェクトは開発・管理用途のみ
2. **一時的な起動**: 長時間の稼働は想定されていません
3. **セキュリティ**: 管理作業時のみの限定的なアクセス

## ファイル構成

```
table_definition/
├── table_definition/          # プロジェクト設定
│   ├── settings.py           # Django設定（整理済み）
│   ├── config.py            # カスタム設定
│   └── urls.py              # URL設定（動的対応）
├── admin_server/             # 管理サーバー機能
│   ├── management/commands/  # カスタムコマンド
│   └── views.py             # ヘルスチェック
├── wix/                     # WIXOSSモデル・管理画面
│   ├── models.py           # データモデル
│   └── admin.py            # 管理画面設定
└── manage.py               # Django管理コマンド
```