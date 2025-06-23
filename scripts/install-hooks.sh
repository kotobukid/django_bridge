#!/bin/bash
# Git hooksのインストールスクリプト
#
# 使い方:
#   ./scripts/install-hooks.sh

set -e

# プロジェクトルートへ移動
cd "$(dirname "$0")/.." || exit 1

# カラー定義
if [ -t 1 ]; then
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    RED='\033[0;31m'
    BLUE='\033[0;34m'
    NC='\033[0m'
else
    GREEN=''
    YELLOW=''
    RED=''
    BLUE=''
    NC=''
fi

echo -e "${BLUE}=== Git Hooks インストーラー ===${NC}"
echo ""

# git-hooksディレクトリの確認
if [ ! -d "git-hooks" ]; then
    echo -e "${RED}エラー: git-hooksディレクトリが見つかりません${NC}"
    exit 1
fi

# .git/hooksディレクトリの確認
if [ ! -d ".git/hooks" ]; then
    echo -e "${RED}エラー: .git/hooksディレクトリが見つかりません${NC}"
    echo "このスクリプトはGitリポジトリのルートで実行してください。"
    exit 1
fi

# インストール対象のフック
HOOKS=(
    "post-merge"
    "pre-commit"
)

# 各フックをインストール
installed=0
skipped=0

for hook in "${HOOKS[@]}"; do
    src="git-hooks/$hook"
    dst=".git/hooks/$hook"
    
    echo -e "${BLUE}処理中: $hook${NC}"
    
    if [ ! -f "$src" ]; then
        echo -e "${YELLOW}警告: $src が見つかりません。スキップします。${NC}"
        skipped=$((skipped + 1))
        continue
    fi
    
    # 既存のフックがある場合は確認
    if [ -f "$dst" ]; then
        echo -e "${YELLOW}既存のフック $hook が見つかりました。${NC}"
        echo -n "上書きしますか？ (y/N): "
        read -r response
        
        if [[ ! "$response" =~ ^[Yy]$ ]]; then
            echo "スキップしました: $hook"
            skipped=$((skipped + 1))
            continue
        fi
        
        # バックアップを作成
        backup="${dst}.backup.$(date +%Y%m%d_%H%M%S)"
        cp "$dst" "$backup"
        echo -e "${GREEN}バックアップを作成しました: $backup${NC}"
    fi
    
    # フックをコピー
    cp "$src" "$dst"
    chmod +x "$dst"
    echo -e "${GREEN}インストール完了: $hook${NC}"
    installed=$((installed + 1))
done

echo ""
echo -e "${BLUE}=== インストール結果 ===${NC}"
echo -e "インストール済み: ${GREEN}$installed${NC} 個"
echo -e "スキップ: ${YELLOW}$skipped${NC} 個"
echo ""

# フックの説明
if [ $installed -gt 0 ]; then
    echo -e "${BLUE}インストールされたフック:${NC}"
    echo ""
    
    if [ -f ".git/hooks/post-merge" ]; then
        echo -e "${GREEN}post-merge${NC}:"
        echo "  - git pull/merge後に[MIGRATION REQUIRED]を含むコミットを検出"
        echo "  - マイグレーションが必要な場合に通知を表示"
        echo ""
    fi
    
    if [ -f ".git/hooks/pre-commit" ]; then
        echo -e "${GREEN}pre-commit${NC}:"
        echo "  - Djangoモデルやマイグレーション関連ファイルの変更を検出"
        echo "  - コミットメッセージに[MIGRATION REQUIRED]の追加を推奨"
        echo ""
    fi
fi

echo -e "${BLUE}完了！${NC}"