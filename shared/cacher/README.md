# Cacherモジュール

このモジュールは、WIXOSS公式サイトからカードデータをキャッシュする機能を提供します。

## アーキテクチャ

モジュールは以下のコンポーネントで構成されています：

### コアモジュール

- **`config`** - 設定定数（URL、遅延時間など）
- **`error`** - カスタムエラー型とResult型エイリアス
- **`search_query`** - WIXOSSカードデータベースへの検索クエリを表現
- **`product_cacher`** - プロダクトのメインキャッシングロジック

### 主な機能

1. **モジュラー設計** - 関心事を個別のモジュールに分離
2. **エラーハンドリング** - panicの代わりに適切なエラー型を使用
3. **キャッシング** - HTTPリクエスト前にローカルキャッシュを確認
4. **レート制限** - リクエスト間の設定可能な遅延

## 使用方法

```rust
use cacher::ProductCacher;
use models::product::Product;
use std::path::PathBuf;

let product = Product { /* ... */ };
let cache_dir = PathBuf::from("./cache");
let cacher = ProductCacher::new(cache_dir, product);

cacher.cache_all_pages().await?;
```

## 設定

主要な設定値は`src/config.rs`にあります：
- `WIXOSS_BASE_URL` - カードリストのベースURL
- `CARDS_PER_PAGE` - 1ページあたりのカード数（21枚）
- `REQUEST_DELAY_SECS` - リクエスト間の遅延（1秒）