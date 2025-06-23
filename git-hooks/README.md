# Git Hooks

このディレクトリには、プロジェクトで使用するGitフックが含まれています。
これらのフックは自動的にはインストールされないため、手動でのセットアップが必要です。

## インストール方法

### 自動インストール（推奨）

```bash
./scripts/install-hooks.sh
```

このスクリプトは以下を実行します：
- 既存のフックのバックアップを作成
- git-hooks/からローカルの.git/hooks/へフックをコピー
- 実行権限を設定

### 手動インストール

特定のフックのみインストールする場合：

```bash
# post-mergeフックのインストール
cp git-hooks/post-merge .git/hooks/
chmod +x .git/hooks/post-merge

# pre-commitフックのインストール
cp git-hooks/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit
```

## フックの説明

### post-merge
- **発動タイミング**: `git pull`や`git merge`の実行後
- **機能**: `[MIGRATION REQUIRED]`を含むコミットメッセージを検出して通知
- **動作**: 
  - ローカル環境: 通知表示後、Enter入力待機（10秒でタイムアウト）
  - CI/CD環境: 通知のみ表示

### pre-commit
- **発動タイミング**: `git commit`の実行時（コミット前）
- **機能**: マイグレーション関連ファイルの変更を検出
- **監視対象**:
  - `table_definition/wix/models.py`
  - `table_definition/wix/models/*.py`
  - `table_definition/*/migrations/*.py`
  - `shared/models/*.rs`
- **動作**: 
  - 該当ファイルの変更を検出したら警告表示
  - `[MIGRATION REQUIRED]`の使用を推奨
  - コミットはブロックしない（推奨のみ）

## カスタマイズ

監視対象のディレクトリを変更する場合は、`pre-commit`ファイル内の`MIGRATION_DIRS`配列を編集してください。

## トラブルシューティング

### フックが動作しない場合
1. 実行権限を確認: `ls -la .git/hooks/`
2. 権限を付与: `chmod +x .git/hooks/*`

### 既存のフックとの競合
- インストールスクリプトは既存のフックをバックアップします
- バックアップファイル: `.git/hooks/<hook-name>.backup.<timestamp>`

## 新規メンバー向けガイド

プロジェクトに参加する際は、以下を実行してください：

```bash
# リポジトリのクローン後
git clone <repository-url>
cd <project-directory>

# フックのインストール
./scripts/install-hooks.sh
```