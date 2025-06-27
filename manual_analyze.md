# 手動CardFeature修正システム実装方針

## 概要
CardFeature検出の精度向上のため、ルールベースの自動検出に加えて人間による手動修正を可能にするシステムを構築する。

## アーキテクチャ

### 1. データモデル設計

**Option A: Cardモデルに直接フィールド追加**
```python
# table_definition/wix/models.py
class Card(models.Model):
    # 既存フィールド...
    fixed_bits1 = models.BigIntegerField(null=True, blank=True)
    fixed_bits2 = models.BigIntegerField(null=True, blank=True)
    fixed_burst_bits = models.BigIntegerField(null=True, blank=True)
    manual_modified = models.BooleanField(default=False)
    manual_modified_at = models.DateTimeField(null=True, blank=True)
```

**Option B: 別モデルとして管理（推奨）**
```python
class CardFeatureOverride(models.Model):
    pronunciation = models.CharField(max_length=200, unique=True, db_index=True)
    fixed_bits1 = models.BigIntegerField()
    fixed_bits2 = models.BigIntegerField()
    fixed_burst_bits = models.BigIntegerField()
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    note = models.TextField(blank=True)  # 修正理由のメモ
```

### 2. FixedDataサーバー (Axum)

**エンドポイント設計**
- `GET /api/overrides` - 全override取得
- `GET /api/overrides/:pronunciation` - 特定カードのoverride取得
- `POST /api/overrides` - override作成/更新
- `DELETE /api/overrides/:pronunciation` - override削除
- `POST /api/analyze/:pronunciation` - 個別カード再解析
- `GET /api/export` - 全データエクスポート（JSON）
- `POST /api/import` - データインポート
- `GET /api/check-consistency` - ルールベースとの一致チェック

**ディレクトリ構造**
```
fixed_data_server/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── overrides.rs
│   │   ├── analyze.rs
│   │   └── import_export.rs
│   ├── models/
│   │   └── feature_override.rs
│   └── db/
│       └── repository.rs
```

### 3. static_generator の改修

```rust
// 修正データの取得と適用
let overrides = load_feature_overrides()?;
let override_map: HashMap<String, (u64, u64, u64)> = overrides
    .into_iter()
    .map(|o| (o.pronunciation, (o.fixed_bits1, o.fixed_bits2, o.fixed_burst_bits)))
    .collect();

// カード生成時に適用
for card in cards {
    let (bits1, bits2, burst_bits) = if let Some(override_data) = override_map.get(&card.pronunciation) {
        *override_data
    } else {
        (card.card_feature_bits1, card.card_feature_bits2, card.burst_feature_bits)
    };
    // ... 以下、コード生成処理
}
```

### 4. wasm_front の拡張

**ルールメンテナンスモード判定**
```rust
fn is_maintenance_mode() -> bool {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let hostname = location.hostname().unwrap();
    
    hostname == "localhost" || hostname == "127.0.0.1" || hostname.contains("local")
}
```

**フラグ編集ページコンポーネント**
```rust
#[component]
pub fn CardFeatureEditor(pronunciation: String) -> impl IntoView {
    // 現在のfeature取得
    let current_features = create_resource(/* ... */);
    
    // feature状態管理
    let feature_states = create_rw_signal(HashMap::new());
    
    // 保存処理
    let save_overrides = create_action(/* ... */);
    
    view! {
        <div class="feature-editor">
            // CardFeatureをカテゴリごとに表示
            <FeatureCategory name="コラボ" features=[/* ... */] />
            <FeatureCategory name="能力" features=[/* ... */] />
            // ...
        </div>
    }
}
```

## 実装順序

1. **Phase 1**: データモデル追加とマイグレーション
2. **Phase 2**: FixedDataサーバーの基本CRUD実装
3. **Phase 3**: static_generatorの改修
4. **Phase 4**: wasm_frontのメンテナンスモード実装
5. **Phase 5**: エクスポート/インポート機能
6. **Phase 6**: 一致チェック機能

## 考慮事項

### パフォーマンス
- static_generator実行時のoverride検索を高速化
- pronunciationインデックスの活用

### データ整合性
- pronunciation変更時の対応
- 複数環境間でのデータ同期

### UI/UX
- CardFeatureの視覚的なグループ化
- 変更履歴の表示
- バルク編集機能の検討

### セキュリティ
- ローカル環境限定のアクセス制御
- CORS設定

## 実装の利点

1. **段階的な精度向上**: ルールベースで対応しきれないケースを個別に修正
2. **複数環境対応**: エクスポート/インポートで異なる環境間でデータ共有
3. **透明性**: manual_modifiedフラグで手動修正の有無を追跡
4. **保守性**: ルール改善後に不要な修正を検出・削除可能

## 実装状況 (2025-06-27 現在)

### ✅ 実装完了

#### Phase 1: データモデル追加とマイグレーション
- CardFeatureOverrideモデル実装済み
- Djangoマイグレーション実行済み

#### Phase 2: FixedDataサーバーの基本CRUD実装  
- 全エンドポイントの基本実装完了
- fixed_data_server (port 8004) 動作確認済み
- APIの正常動作確認済み

#### Phase 3: static_generatorの改修
- override適用ロジック実装済み
- shared/webapp/repositories/card.rs に組み込み済み
- Card::with_feature_override()メソッド追加済み

#### Phase 4: wasm_frontのメンテナンスモード実装
- メンテナンスモード判定機能実装済み
- フィーチャー編集ページ完全実装済み
- CardItemコンポーネントに編集リンク追加済み
- 実際のCardFeature定義を使用したUI構築済み
- FeatureTagの数値順ソート実装済み

### ⚠️ 未実装項目

#### Phase 5: エクスポート/インポート機能
- `GET /api/export` - エンドポイント定義済みだが実装未完了
- `POST /api/import` - エンドポイント定義済みだが実装未完了
- 複数環境間でのデータ同期機能が未完成

#### Phase 6: 個別解析機能
- `POST /api/analyze/:pronunciation` - エンドポイント定義済みだが実装未完了
- 手動修正後の個別カード再解析機能が未実装

### 🔧 実装時に解決した技術的課題

1. **SQLクエリエラー**: created_at制約違反 → CURRENT_TIMESTAMPで解決
2. **インポートエラー**: feature::CardFeature → feature::feature::CardFeatureで解決
3. **ビット変換ロジックエラー**: OR条件 → datapackと同じ条件分岐ロジックで解決
4. **フロントエンド初期化**: 実際のカードデータからの自動フィーチャー設定を実装

## 今後の懸念事項

### 1. データ同期の課題
**問題**: 複数の開発環境間でCardFeatureOverrideデータを同期する手段が未実装
**影響**: チーム開発時に手動修正データが各環境で異なる状態になる可能性
**対策**: 
- エクスポート/インポート機能の早期実装
- git管理可能なJSON形式でのデータ保存検討

### 2. パフォーマンスの懸念
**問題**: static_generator実行時にCardFeatureOverrideテーブル全件読み込み
**影響**: データ量増加時の生成時間増大
**対策**:
- pronunciationインデックス活用の最適化
- 差分更新機能の検討

### 3. データ整合性の課題
**問題**: pronunciationが変更された場合のoverride orphan化
**影響**: 無効なoverride データが蓄積される可能性
**対策**:
- pronunciation変更検出機能
- orphanデータのクリーンアップ機能

### 4. UI/UXの改善余地
**問題**: 大量のCardFeatureを一画面で管理する使いにくさ
**影響**: 編集効率の低下
**対策**:
- 検索・フィルタ機能の追加
- バルク編集機能の検討
- 変更履歴表示機能

### 5. 運用上の課題
**問題**: 手動修正の理由やタイミングが追跡困難
**影響**: なぜその修正が必要だったかの情報喪失
**対策**:
- noteフィールドの積極活用促進
- 修正理由のテンプレート化
- 変更ログ機能の追加

### 6. セキュリティ考慮事項
**問題**: fixed_data_server (port 8004) がローカル以外からもアクセス可能
**影響**: 本番環境での意図しない修正リスク
**対策**:
- IP制限の強化
- 認証機能の追加検討

## 優先度付き今後のタスク

### 高優先度
1. **エクスポート/インポート機能実装** - チーム開発で必須
2. **orphanデータクリーンアップ機能** - データ整合性確保

### 中優先度  
3. **個別解析機能実装** - 修正後の検証用
4. **UI改善（検索・フィルタ）** - 操作性向上

### 低優先度
5. **変更履歴機能** - 運用改善
6. **認証機能** - セキュリティ強化