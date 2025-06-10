#!/bin/sh
# プロモーションカード一括取得
echo "Starting promotion cards download..."

# PRカードは製品コード不要
cargo make scraper pr --cache-dir ./custom_cache --min-delay 500 --max-delay 2000

echo "Promotion cards download completed!"