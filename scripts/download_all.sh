#!/bin/sh
# 全カード一括取得スクリプト

# カラー出力用の定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ヘルプメッセージ
show_help() {
    echo "Usage: $0 [OPTION]"
    echo "Download WIXOSS card data by product type"
    echo ""
    echo "Options:"
    echo "  all       Download all product types (booster, starter, sp, pr)"
    echo "  booster   Download only booster products"
    echo "  starter   Download only starter products"
    echo "  sp        Download only special products"
    echo "  pr        Download only promotion products"
    echo "  help      Show this help message"
    echo ""
    echo "Example:"
    echo "  $0 all      # Download everything"
    echo "  $0 booster  # Download only booster products"
}

# 引数チェック
if [ $# -eq 0 ]; then
    echo -e "${RED}Error: No argument provided${NC}"
    show_help
    exit 1
fi

# メイン処理
case "$1" in
    all)
        echo -e "${BLUE}===== Starting download for ALL product types =====${NC}"
        echo ""
        
        echo -e "${GREEN}[1/4] Downloading BOOSTER products...${NC}"
        sh ./booster.bat
        echo ""
        
        echo -e "${GREEN}[2/4] Downloading STARTER products...${NC}"
        sh ./scripts/starter.sh
        echo ""
        
        echo -e "${GREEN}[3/4] Downloading SPECIAL products...${NC}"
        sh ./scripts/special.sh
        echo ""
        
        echo -e "${GREEN}[4/4] Downloading PROMOTION products...${NC}"
        sh ./scripts/promotion.sh
        echo ""
        
        echo -e "${BLUE}===== All downloads completed! =====${NC}"
        ;;
        
    booster)
        echo -e "${BLUE}===== Downloading BOOSTER products only =====${NC}"
        sh ./booster.bat
        echo -e "${GREEN}Booster download completed!${NC}"
        ;;
        
    starter)
        echo -e "${BLUE}===== Downloading STARTER products only =====${NC}"
        sh ./scripts/starter.sh
        echo -e "${GREEN}Starter download completed!${NC}"
        ;;
        
    sp)
        echo -e "${BLUE}===== Downloading SPECIAL products only =====${NC}"
        sh ./scripts/special.sh
        echo -e "${GREEN}Special download completed!${NC}"
        ;;
        
    pr)
        echo -e "${BLUE}===== Downloading PROMOTION products only =====${NC}"
        sh ./scripts/promotion.sh
        echo -e "${GREEN}Promotion download completed!${NC}"
        ;;
        
    help)
        show_help
        ;;
        
    *)
        echo -e "${RED}Error: Unknown option '$1'${NC}"
        show_help
        exit 1
        ;;
esac