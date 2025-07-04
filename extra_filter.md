# LBフィルター追加検討メモ

## 要求仕様
wasm_frontに検索項目「LBあり」「LBなし」「指定なし」を追加したい。

### 実装方針
- CardExport及びdatapackクレートの生成rsコードに項目追加
- フィールド名: `has_lb` (実際は既存の`has_burst`を流用)
- データ型: `u8`
- 取りうる値:
  - `0`: 指定なし (ルリグ、アーツ等)
  - `1`: LBあり (シグニ・スペルでburst_textが存在)
  - `2`: LBなし (シグニ・スペルでburst_textが空)
- 対象カードタイプ: シグニ(5)、スペル(6)のみ適用、その他は0

## 現状分析結果

### 既存has_burstフィールドの状況
- **位置**: CardExportの10番目のフィールド (CardStaticタプルのindex 10)
- **現在の実装**: 手動設定方式（データベース値をそのまま使用）
- **使用箇所**: 表示制御のみ、フィルタリング機能は未実装
- **Django定義**: `IntegerField`、コメント「バーストあり1，なし2、無関係0」

### カードタイプ定数
- **シグニ**: card_type = 5
- **スペル**: card_type = 6
- **シグニクラフト**: card_type = 10
- **スペルクラフト**: card_type = 11

## パフォーマンス・拡張性分析

### パフォーマンス比較
| 方式 | アクセス方法 | パフォーマンス | メモリ使用量 |
|------|-------------|---------------|-------------|
| u8単一フィールド | `card.has_burst == 1` | 1つの整数比較 | 1バイト |
| boolean2個 | 2つの別フィールド + 論理演算 | 若干劣る | 2バイト |

**結論**: u8の方が僅かに高速だが、実用上の差は無視できる

### 拡張性比較
| 拡張ケース | u8アプローチ | boolean2個アプローチ |
|------------|--------------|---------------------|
| LB有/無/不明 | `0,1,2` | `has_lb, lb_irrelevant` |
| + コスト有無 | `0-5`の6段階 | `has_lb, has_cost, lb_irrelevant, cost_irrelevant` |
| + レベル制限 | `0-11`の12段階 | さらにboolean追加が必要 |

### 既存パターンとの一貫性
- **format**: `(1, 3, 7)` でビットフラグ組み合わせ
- **rarity**: 13種類の値を管理
- **card_type**: 16種類のカード種別

## 推奨実装方針

**u8単一フィールド（既存has_burstの流用）を推奨**

### 理由
1. **パフォーマンス**: 僅かに高速
2. **拡張性**: より多くの分類軸に対応可能
3. **一貫性**: 既存の`format`, `card_type`パターンと統一
4. **コード簡潔性**: 1つの値での状態管理

### 自動計算ロジック案
```rust
// Card::to_rust_code()での実装例
let has_burst = match card_type {
    5 | 6 | 10 | 11 => { // シグニ、スペル、クラフト系
        if !burst_text.is_empty() { 1 } // LBあり
        else { 2 }                      // LBなし
    },
    _ => 0 // 指定なし（ルリグ、アーツなど）
};
```

### 判定基準の改善
- **現在の提案**: `burst_bits == 0` で判定
- **推奨**: `burst_text.is_empty()` で判定
- **理由**: より正確（burst_bitsは機能フラグなので、0でもテキストがある場合がある）

## 実装箇所

1. **static_generator**: `Card::to_rust_code()`での自動計算ロジック追加
2. **datapack**: `src/filter.rs`にフィルタリング関数追加
3. **wasm_front**: UIコンポーネントにフィルター選択肢追加

## 将来の拡張例
```rust
// 値の拡張例
// 0: 指定なし（ルリグ、アーツ等）
// 1: LBあり
// 2: LBなし
// 3: コストありLBあり
// 4: コストありLBなし
// 5: レベル制限ありLBあり
// ...
```

## データフロー
```
Database(wix_card) 
→ static_generator(自動計算) 
→ datapack/src/gen/cards.rs 
→ CardExport.has_burst 
→ wasm_front(フィルタリング)
```

---
**作成日**: 2025-01-10
**検討者**: Claude Code
**ステータス**: 設計完了、実装待ち