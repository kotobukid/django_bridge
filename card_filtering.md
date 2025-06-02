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

## 今後の対応方針

### 1. WASM側修正（推奨）
- `CardExport`に`feature_bits1`、`feature_bits2`を追加
- 新しいAND条件専用のWASM関数を実装
- ビット演算ロジックの見直し

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