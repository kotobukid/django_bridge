# Rust-Django Integration Template

## 概要

このプロジェクトテンプレートは、Rustプロジェクトの中で[**Django**](https://www.djangoproject.com/)
の強力なマイグレーション機能や管理画面を利用する選択肢を提供します。Rustだけではモデル定義やマイグレーション管理が複雑になりがちな場合に、Djangoの強力なツールを補完的に活用できます。

### 主な機能

1. **Djangoマイグレーションツールの活用**  
   Djangoの`makemigrations`や`migrate`コマンドを直接実行し、データベーススキーマ管理を行います。

2. **Rustプロジェクトとの同期**  
   マイグレーション後のDjangoモデルの変更をRustコードで利用可能な状態に同期します（`cargo run --bin syncdb`）。

3. **Django管理画面の利用**  
   必要に応じてDjangoの自動生成管理画面（`runserver`）を活用し、データ管理を効率化。

4. **Rustのビジネスロジックとの統合**  
   Djangoモデルをベースに、Rustで追加のロジックやデータ操作を行うことができます。

---

## 動作概要

このプロジェクトでは以下のワークフロ―を採用します：

1. **Djangoでモデルとマイグレーションを管理**  
   Pythonコードでモデルを定義するとともに、`manage.py makemigrations`や`migrate`などのDjango標準コマンドでデータベース定義を操作します。

   ```bash
   python manage.py makemigrations
   python manage.py migrate
   ```

2. **Rustで変更を同期**  
   Djangoによるスキーマ変更後、Rustコードベースにその変更を同期させます。

   ```bash
   cargo run --bin syncdb --features="generator-deps"
   ```

   これにより、Django側のモデルがRustコード内で利用可能な形（例: `CardDb` や `TagDb` 構造体）になります。

3. **Django管理画面の活用（必要に応じて）**  
   データ管理が必要であれば、Djangoの`runserver`コマンドを利用して管理画面を起動します。

---

## 必要な環境

- Rust 1.84.1 以上
- Python 3.10 以上
- Django 4.x
- 必要なデータベース（PostgreSQL, MySQL, SQLite など）

---

## インストールとセットアップ

### 環境構築

1. **Django環境のセットアップ**  
   必要なPython依存ライブラリをインストールしてください。

   ```bash
   pip install django psycopg2-binary
   ```

2. **Rustプロジェクトのセットアップ**  
   このリポジトリをクローンし、依存ライブラリを解決します。

   ```bash
   git clone <REPOSITORY_URL>
   cd <PROJECT_DIRECTORY>
   cd sqlm
   cargo build
   ```

### 初期セットアップ

1. **Djangoマイグレーション**  
   Djangoで初期スキーマを適用します。

   ```bash
   cd table_definition
   python manage.py makemigrations
   python manage.py migrate
   ```

2. **Rustコードの同期**  
   マイグレーションが完了したら、Rustで同期を行います。

   ```bash
   cargo run --bin syncdb --features="generator-deps"
   ```

---

## 主なコマンド

### Django関連コマンド

- **`manage.py makemigrations`**  
  モデル変更に基づくマイグレーションファイルを生成します。
- **`manage.py migrate`**  
  マイグレーションをデータベースに適用します。
- **`manage.py runserver`**  
  開発用サーバを起動し、Django管理画面にアクセス可能にします。

Djangoのカスタムコマンド作成機能も便利です。
[カスタム django-admin コマンドの作り方](https://docs.djangoproject.com/ja/5.1/howto/custom-management-commands/)

### Rust関連コマンド

- **`cargo run --bin syncdb --features="generator-deps"`**  
  DjangoモデルをRustプロジェクトに同期します。

---

## モデルのサンプル

Django側で以下のモデルを定義します（例: `models.py`）：

```python
from django.db import models


class Card(models.Model):
    name = models.CharField(max_length=256, default="taro")
    created_at = models.DateTimeField(auto_now_add=True)
    bool1 = models.BooleanField(default=False)
    option1 = models.CharField(max_length=128, null=True, blank=True)
```

これをRust側に同期させると、自動的に以下のようなコードが生成されます（例: `django_models.rs`）：

```rust
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct CardDb {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub bool1: bool,
    pub option1: Option<String>,
}
```

さらに、`models.rs`にカスタムロジックを追加可能です(RustのOrphansルールの都合上、自動生成されたCardDb構造体自信にトレイト実装を記述するのは難儀するため、別ファイル内でNewTypeパターンを用いて派生させます)：

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Card(pub CardDb);

impl From<CardDb> for Card {
   fn from(db: CardDb) -> Self {
      Self(db)
   }
}

impl std::ops::Deref for Card {
   type Target = CardDb;

   fn deref(&self) -> &Self::Target {
      &self.0
   }
}

impl Card {
    pub fn display_info(&self) -> String {
        format!("{} - {}", self.name, self.option1.clone().unwrap_or_default())
    }
}
```

---

## 利用の流れ

1. Djangoでモデルを編集 → `makemigrations` → `migrate`
2. Rustで同期コマンドを実行 → 必要に応じて追加のロジックを記述
3. 必要ならDjango管理画面でデータ操作を行う

---

## ライセンス

MITライセンスのもと提供されています。

--- 