# カードフィルタリング機能の現状と課題

## 概要
WIXOSSトレーディングカードゲーム管理システムのNuxt.jsフロントエンドにおけるカードフィルタリング機能の実装状況をまとめる。

## 実装済み機能

### 1. UI改善
- ✅ Windowsアプリ風のメニューバー実装
- ✅ ナビゲーションバーに特徴フィルターを統合
- ✅ カラーフィルターの視覚的フィードバック
- ✅ 親カテゴリーへの選択状態インジケーター（緑色背景+ドット）
- ✅ プロフェッショナルなデザインとレイアウト

### 2. カラーフィルター
- ✅ 色選択時の視覚フィードバック
- ✅ カラーフィルターとフィーチャーフィルターの連携
- ✅ Clear All機能による全フィルターリセット

### 3. フィーチャーフィルター（基本動作）
- ✅ チェック追加時のフィルタリング実行
- ✅ チェック解除時のフィルタリング実行
- ✅ 状態管理とイベント処理

## 確認された動作

### テストケース1: 単一フィーチャー
```
アサシンにチェック → 58件表示 ✅
Sランサーにチェック → 27件表示 ✅
```

### テストケース2: 複数フィーチャー選択
```
アサシン（58件）+ Sランサー（27件）→ 27件表示 ❌
期待値: AND条件で大幅減少（例：3-5件程度）
実際: Sランサー単独と同じ結果
```

### テストケース3: フィーチャー解除
```
アサシン + Sランサー + ダメージ の状態から
アサシンのチェックを外す → 58件表示 ❌
期待値: Sランサー + ダメージ の結果
実際: アサシン単独の結果と同じ
```

## 技術的課題

### 1. WASM実装の制限
**問題**: `CardExport`構造体に`feature_bits1`、`feature_bits2`が含まれていない
```rust
// datapack/src/lib.rs:70-71
// i64, // feature_bits1  ← コメントアウト
// i64, // feature_bits2  ← コメントアウト
```

**影響**: JavaScriptサイドでのビット演算ベースAND条件が不可能

### 2. WASM fetch_by_f_bits の動作
**現在の実装**: OR条件での動作
```rust
// datapack/src/lib.rs:208
(feature_bits1 & bit1) != 0  // ← OR条件（いずれかのビットが一致）
```

**期待される動作**: AND条件
```rust
(feature_bits1 & bit1) == bit1  // ← AND条件（全ビットが一致）
```

### 3. ID集合交差アプローチの限界
**試行したアプローチ**: 
- 各フィーチャーで`fetch_by_f_shifts`を個別実行
- カードIDの集合を作成し交差演算
- 結果のIDから最終カード一覧を構築

**結果**: まだ期待通りに動作していない

## デバッグログ出力例

### 正常動作時（単一フィーチャー）
```
=== Feature Filter Applied ===
Feature: アサシン
Action: ADDING
Current selected count BEFORE: 0
Current selected count AFTER: 1
Selected features: ['アサシン_6_0']
Applying multiple feature filter
Final card count: 58
```

### 問題動作時（複数フィーチャー）
```
=== Feature Filter Applied ===
Feature: Sランサー
Action: ADDING
Current selected count BEFORE: 1
Current selected count AFTER: 2
Selected features: ['アサシン_6_0', 'Sランサー_12_0']
Combined bits: bits1=4160, bits2=1
Cards from WASM: 84 ← OR条件の結果
Final card count: 84
```

## アーキテクチャ

### フロントエンド構成
```
card.vue (メインページ)
├── NavBar.vue (フィーチャーメニュー)
├── ColorSelector.vue (カラーフィルター)
└── カード一覧表示

状態管理:
- selectedFeatures: Map<string, any> (選択されたフィーチャー)
- color_bits: computed (選択されたカラーのビットフラグ)
```

### WASM連携
```
datapack.js (WASM)
├── fetch_by_f_bits(bit1, bit2) → OR条件検索
├── fetch_by_f_shifts(shift1, shift2) → 単一フィーチャー検索
└── feature_conditions() → フィーチャー定義取得
```

## WASMアーキテクチャの詳細

### 特殊なデータ管理アプローチ
このWASMモジュールは、以下の特殊なアーキテクチャを採用している：

1. **データの事前埋め込み**
   - RDBのすべてのカードデータを、ビルド時にRustコードとして生成
   - 生成されたコードは`datapack/src/gen/cards.rs`に配置
   - WASMバイナリにデータが直接含まれるため、実行時のデータベースアクセスが不要

2. **CardExport型の必要性**
   - `CardExport`構造体は`#[wasm_bindgen]`属性が必要
   - TypeScriptの型定義（d.ts）に含まれることで、フロントエンドでの型安全性を確保
   - WASMとJavaScript間のデータ受け渡しインターフェースとして機能

### BigInt使用の必要性

#### JavaScriptの数値制限
- JavaScriptのNumber型は IEEE 754 倍精度浮動小数点数
- 安全に扱える整数の最大値は 2^53 - 1 (Number.MAX_SAFE_INTEGER)
- 53ビットを超える整数は精度が失われる

#### ビットフラグとBigInt
```javascript
// NG: 53ビットを超えるとデータが破壊される
const featureBits = 9007199254740993; // 2^53 + 1

// OK: BigIntを使用
const featureBits = BigInt("9007199254740993");
```

#### 実装例
```javascript
// ユーザー入力（文字列）からBigIntへの変換
const userInput = "1234567890123456789";
const bits1 = BigInt(userInput);
const bits2 = BigInt("0");

// WASM関数への引き渡し
const results = datapack.fetch_by_f_bits(bits1, bits2);
```

### フィーチャービットフラグの構造
- 各カードは最大128個のフィーチャーフラグを持つ（64bit × 2）
- `feature_bits1`: 最初の64個のフィーチャー
- `feature_bits2`: 次の64個のフィーチャー
- ビット演算により高速な検索が可能

## 実装した修正内容（2025年1月）

### 1. WASM側修正
- ✅ `CardExport`に`feature_bits1`、`feature_bits2`を追加（getter実装）
- ✅ `Vec<CardExport>`を返すように関数を修正
- ✅ `fetch_by_combined_bits`関数でAND/OR条件を明示的に指定可能

### 2. フロントエンド側修正
- ✅ `applyMultipleFeatureFilter`を`fetch_by_combined_bits`を使用するように変更
- ✅ ビットマスクを直接計算してWASMに渡す実装
- ✅ デバッグ情報の追加（feature_bits1/2の表示）

### 3. 修正後の動作
- 複数フィーチャー選択時：すべての条件を満たすカードのみ表示（AND条件）
- チェック解除時：残りの選択されたフィーチャーで再フィルタリング
- feature_bits1/2がJavaScriptから参照可能になり、デバッグが容易に

## 今後の対応方針

### 1. WASM側修正（完了）
- ✅ `CardExport`に`feature_bits1`、`feature_bits2`を追加
- ✅ 新しいAND条件専用のWASM関数を実装
- ✅ ビット演算ロジックの見直し

### 2. フロントエンド側回避策
- ID集合交差アプローチの改良
- 段階的フィルタリングの実装

### 3. 仕様の明確化
- AND/OR条件の動作仕様を確定
- ユーザビリティテストの実施

## 関連ファイル

### フロントエンド
- `front/pages/card.vue` - メインカードページ
- `front/components/NavBar.vue` - フィーチャーメニュー
- `front/components/ColorSelector.vue` - カラーフィルター
- `front/stores/card_js.ts` - カード状態管理

### バックエンド
- `datapack/src/lib.rs` - WASM実装
- `shared/feature/src/feature.rs` - フィーチャー定義

## GitHub Issue
Issue #6: フロントエンド機能統合・整理タスク
https://github.com/kotobukid/django_bridge/issues/6