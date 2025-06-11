# Leptos ベストプラクティス ガイド

本ドキュメントは、WIXOSS Card Database プロジェクトで得られたLeptosの実戦的なベストプラクティスをまとめたものです。

Author: Claude Code

## 目次

1. [反応性システムの理解と活用](#反応性システムの理解と活用)
2. [コンポーネント設計](#コンポーネント設計)
3. [状態管理](#状態管理)
4. [クロージャーとライフタイム管理](#クロージャーとライフタイム管理)
5. [動的レンダリングパターン](#動的レンダリングパターン)
6. [エラーハンドリングと対処法](#エラーハンドリングと対処法)
7. [パフォーマンス最適化](#パフォーマンス最適化)

## 反応性システムの理解と活用

### Signal, Memo, Effect の使い分け

```rust
// ✅ 基本的な状態には signal() を使用
let (count, set_count) = signal(0);

// ✅ 計算値には Memo を使用
let doubled = Memo::new(move |_| count.get() * 2);

// ✅ 副作用には Effect を使用
Effect::new(move || {
    logging::log!("Count changed to: {}", count.get());
});
```

### RwSignal vs Signal の選択

```rust
// ✅ 複数の場所で更新が必要な場合は RwSignal
let shared_state = RwSignal::new(initial_value);

// ✅ 単一の場所でのみ更新する場合は signal()
let (local_state, set_local_state) = signal(initial_value);
```

### 反応性の伝播

```rust
// ❌ 間違い: 反応性が失われる
let value = signal_value.get();
view! { <div>{value}</div> }

// ✅ 正解: 反応性を保持
view! { <div>{move || signal_value.get()}</div> }
```

## コンポーネント設計

### Props の設計

```rust
// ✅ 推奨: 型安全性と柔軟性を両立
#[component]
pub fn MyComponent(
    #[prop(into)] label: String,           // String を受け取り、自動変換
    #[prop(into)] is_active: Signal<bool>, // Signal を受け取り
    #[prop(into)] on_click: Callback<()>,  // コールバック
) -> impl IntoView {
    // 実装
}
```

### コンポーネントの責務分離

```rust
// ✅ 単一責務の原則
#[component]
pub fn ProductSelector(product_filter: RwSignal<ProductFilter>) -> impl IntoView {
    // 商品選択のみに集中
}

#[component]
pub fn FeatureSelector(selected_features: RwSignal<HashMap<String, bool>>) -> impl IntoView {
    // 機能選択のみに集中
}
```

## 状態管理

### 状態の配置戦略

```rust
// ✅ 親コンポーネントで状態を管理し、props で渡す
#[component]
pub fn ParentComponent() -> impl IntoView {
    let shared_state = RwSignal::new(initial_value);
    
    view! {
        <ChildA state=shared_state />
        <ChildB state=shared_state />
    }
}
```

### 強制再描画パターン

```rust
// ✅ 反応性が機能しない場合の対処法
let (force_update_key, set_force_update_key) = signal(0u32);

// 強制更新が必要な場所で
set_force_update_key.update(|k| *k += 1);

// コンポーネントで監視
view! {
    {move || {
        let _key = force_update_key.get(); // キーを監視
        view! { <ComplexComponent /> }
    }}
}
```

## クロージャーとライフタイム管理

### よくあるエラーとその対処法

#### 1. "closure invoked recursively or after being dropped"

```rust
// ❌ 問題のあるコード
{items.into_iter().map(|item| {
    let name = item.name.clone();
    view! { <div>{name}</div> }
}).collect::<Vec<_>>()}

// ✅ 解決策: クロージャー外でデータを準備
let item_views = items.iter().map(|item| {
    let name = item.name.clone();
    view! { <div>{name}</div> }
}).collect::<Vec<_>>();

view! { {item_views} }
```

#### 2. "expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`"

```rust
// ❌ 問題のあるコード
{data.into_iter().map(|item| /* ... */).collect_view()}

// ✅ 解決策1: iter() を使用
{data.iter().map(|item| /* ... */).collect_view()}

// ✅ 解決策2: 事前にクローン
let data_clone = data.clone();
{data_clone.into_iter().map(|item| /* ... */).collect_view()}

// ✅ 解決策3: 構造体化して管理
#[derive(Clone)]
struct Item { /* fields */ }

let items: Vec<Item> = /* データ準備 */;
{items.iter().map(|item| /* ... */).collect_view()}
```

### Show コンポーネント内でのデータ使用

```rust
// ❌ 問題: データが消費される
<Show when=move || condition.get()>
    {data.into_iter().map(/* ... */).collect_view()}
</Show>

// ✅ 解決策: if let パターンを使用
<Show when=move || condition.get()>
    {if let Some(items) = data_source.get(&key) {
        items.iter().map(|item| {
            // アイテム処理
        }).collect_view().into_any()
    } else {
        view! { <div>"No data"</div> }.into_any()
    }}
</Show>
```

## 動的レンダリングパターン

### ドロップダウン/アコーディオンの実装

```rust
#[component]
pub fn AccordionItem(
    title: String,
    is_open: Memo<bool>,
    on_toggle: Callback<()>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="border rounded-lg">
            <button 
                class={move || {
                    let base = "w-full px-4 py-3 text-left font-medium";
                    let state = if is_open.get() { "bg-gray-50" } else { "bg-white" };
                    format!("{} {}", base, state)
                }}
                on:click=move |_| on_toggle.run(())
            >
                <div class="flex items-center justify-between">
                    <span>{title}</span>
                    <span class={move || if is_open.get() { "transform rotate-180" } else { "" }}>
                        "▼"
                    </span>
                </div>
            </button>
            
            <Show when=move || is_open.get()>
                <div class="border-t bg-gray-50">
                    {children()}
                </div>
            </Show>
        </div>
    }
}
```

### オーバーレイ/モーダルの実装

```rust
// ✅ 推奨パターン: 単純で確実
<Show when=move || show_overlay.get()>
    <div 
        class="fixed inset-0 z-50 flex items-center justify-center"
        style="background-color: rgba(0, 0, 0, 0.75);"  // opacity継承問題を回避
        on:click=move |_| set_show_overlay.set(false)
    >
        <div 
            class="bg-white rounded-lg shadow-lg max-w-4xl max-h-[80vh] w-full mx-4"
            on:click=|e| e.stop_propagation()  // バブリング防止
        >
            {/* コンテンツ */}
        </div>
    </div>
</Show>
```

### チェックボックス状態の管理

```rust
// ✅ 正しいチェックボックス実装
let is_selected = Signal::derive(move || {
    filter_state.read().is_selected(item_id)
});

view! {
    <input 
        type="checkbox"
        checked=is_selected  // Signal を直接渡す
        on:change=move |_| {
            filter_state.update(|state| state.toggle(item_id));
        }
    />
}
```

## エラーハンドリングと対処法

### よくあるコンパイルエラー

#### 1. "cannot borrow as mutable"

```rust
// ❌ 問題
let mut state = create_signal(initial);
state.update(/* ... */); // 借用エラー

// ✅ 解決策: RwSignal を使用
let state = RwSignal::new(initial);
state.update(/* ... */); // OK
```

#### 2. "moved value used here after move"

```rust
// ❌ 問題
let data = vec![/* ... */];
let closure1 = move || data.len();  // data が move される
let closure2 = move || data.is_empty(); // エラー: data は既に move されている

// ✅ 解決策: 必要な分だけクローン
let data = vec![/* ... */];
let data_len = data.len();
let data_is_empty = data.is_empty();
let closure1 = move || data_len;
let closure2 = move || data_is_empty;
```

### デバッグ手法

```rust
// ✅ デバッグ用ログ
Effect::new(move || {
    logging::log!("State changed: {:?}", state.get());
});

// ✅ 条件付きレンダリングのデバッグ
<Show when=move || {
    let condition = some_condition.get();
    logging::log!("Condition: {}", condition);
    condition
}>
    {/* content */}
</Show>
```

## パフォーマンス最適化

### 不要な再計算の回避

```rust
// ❌ 毎回計算される
view! { 
    <div>{expensive_calculation()}</div> 
}

// ✅ メモ化して最適化
let memoized_value = Memo::new(move |_| expensive_calculation());
view! { 
    <div>{memoized_value.get()}</div> 
}
```

### イベントハンドラーの最適化

```rust
// ❌ 毎回新しいクロージャーが作成される
view! {
    <button on:click=move |_| handle_click(item.id)>
        "Click"
    </button>
}

// ✅ 安定したコールバックを使用
let handle_click = Callback::new(move |id: u32| {
    // 処理
});

view! {
    <button on:click=move |_| handle_click.run(item.id)>
        "Click"
    </button>
}
```

## プロジェクト固有のパターン

### 静的データの活用

```rust
// ✅ WASM 向け: 静的データを活用してバンドルサイズ削減
fn get_feature_data() -> (Vec<FeatureTag>, HashMap<String, Vec<CardFeature>>) {
    let feature_map = datapack::feature_conditions(); // WASM から取得
    // データ変換処理
}
```

### フィルタリング状態の管理

```rust
// ✅ 複合フィルタリングパターン
Effect::new(move || {
    let color = color_filter.get();
    let features = feature_filter.read();
    let products = product_filter.read();
    
    let filtered = apply_all_filters(&original_data, &color, &features, &products);
    set_filtered_data.set(filtered);
});
```

## 困った時のチェックリスト

### コンパイルエラーが出た時

1. **FnOnce エラー**: `iter()` を `into_iter()` に変更、または事前にクローン
2. **借用エラー**: `RwSignal` の使用を検討
3. **ライフタイム エラー**: 必要な値を事前に取得してクロージャーに渡す
4. **move エラー**: クローンするか、参照で済む設計に変更

### 反応性が機能しない時

1. **強制再描画**: キーを使った強制更新を実装
2. **Signal の確認**: 正しく `Signal::derive` を使用しているか
3. **Effect の確認**: 依存関係が正しく設定されているか

### UI が期待通りに動作しない時

1. **イベント伝播**: `stop_propagation()` が必要かチェック
2. **CSS 継承**: `opacity` vs `rgba()` の使い分け
3. **条件分岐**: `Show` コンポーネントの条件が正しいかチェック

このガイドは、実際のプロジェクトで遭遇した問題と解決策をベースに作成されています。新しいパターンや問題が発見されたら、随時このドキュメントを更新してください。