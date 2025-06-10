#!/bin/bash
# Download ALL WIXOSS products (booster, starter, special)
# Usage: bash download_all_products.sh [booster|starter|sp|all]

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Function to download products
download_products() {
    local product_type=$1
    shift
    local codes=("$@")
    
    echo ""
    print_info "Starting download of ${#codes[@]} ${product_type} products..."
    echo "=================================================="
    
    local success_count=0
    local fail_count=0
    local current=1
    
    for code in "${codes[@]}"; do
        echo ""
        print_info "[${current}/${#codes[@]}] Downloading ${code}..."
        
        cargo run --release -p wxdb-scraper -- ${product_type} "${code}"
        
        if [ $? -eq 0 ]; then
            print_success "Successfully downloaded ${code}"
            ((success_count++))
        else
            print_error "Failed to download ${code}"
            ((fail_count++))
        fi
        
        ((current++))
        
        # Wait between downloads to avoid overwhelming the server
        if [ "${code}" != "${codes[-1]}" ]; then
            echo "Waiting 5 seconds before next download..."
            sleep 5
        fi
    done
    
    echo ""
    echo "=================================================="
    print_info "${product_type} downloads complete!"
    print_success "Success: ${success_count}"
    if [ ${fail_count} -gt 0 ]; then
        print_error "Failed: ${fail_count}"
    fi
    print_info "Total: ${#codes[@]}"
    
    return ${fail_count}
}

# Define all product codes
BO_CODES=(
    "WX25-CP1" "WX24-P4" "WX24-P3" "WX24-P2" "WX24-P1"
    "WXDi-P16" "WXDi-P15" "WXDi-CP02" "WXDi-P14" "WXDi-P13"
    "WXDi-P12" "WXDi-CP01" "WXDi-P11" "WXDi-P10" "WXDi-P09"
    "WXDi-P08" "WXDi-P07" "WXDi-P06" "WXDi-P05" "WXDi-P04"
    "WXDi-P03" "WXDi-P02" "WXDi-P01" "WXDi-P00" "WXK-11"
    "WXK-10" "WXK-09" "WXK-08" "WXK-07" "WXK-06"
    "WXK-05" "WXK-04" "WXK-03" "WXK-02" "WXK-01"
    "WXEX-2" "WXEX-1" "WX-22" "WX-21" "WX-20"
    "WX-19" "WX-18" "WX-17" "WX-16" "WX-15"
    "WX-14" "WX-13" "WX-12" "WX-11" "WX-10"
    "WX-09" "WX-08" "WX-07" "WX-06" "WX-05"
    "WX-04" "WX-03" "WX-02" "WX-01"
)

ST_CODES=(
    "WX25-CD1" "WX24-D5" "WX24-D4" "WX24-D3" "WX24-D2"
    "WX24-D1" "WXDi-D09" "WXDi-D08" "WXDi-D07" "WXDi-D06"
    "WXDi-D05" "WXDi-D04" "WXDi-D03" "WXDi-D02" "WXDi-D01"
    "WDA-F05" "WDA-F04" "WDA-F03" "WDA-F02" "WDA-F01"
    "WDK-17" "WDK-16" "WDK-15" "WDK-14" "WDK-13"
    "WDK-12" "WDK-F05" "WDK-F04" "WDK-F03" "WDK-F02"
    "WDK-F01" "WDK-11" "WDK-10" "WDK-09" "WDK-08"
    "WDK-07" "WDK-06" "WDK-05" "WDK-04" "WDK-03"
    "WDK-02" "WDK-01" "WXD-23" "WXD-22" "WXD-21"
    "WXD-20" "WXD-19" "WXD-18" "WXD-17" "WXD-16"
    "WXD-15" "WXD-14" "WXD-13" "WXD-12" "WXD-11"
    "WXD-10" "WXD-09" "WXD-08" "WXD-07" "WXD-06"
    "WXD-05" "WXD-04" "WXD-03" "WXD-02" "WXD-01"
)

SP_CODES=(
    "SPDi01" "SPDi02" "SPDi03" "SPDi04" "SPDi05"
    "SPDi06" "SPDi07" "SPDi08" "SPDi09" "SPDi10"
    "SPDi11" "SPDi12" "SPDi13" "SPDi14" "SPDi15"
    "SPDi16" "SPDi17" "SPDi19" "SPDi20" "SPDi21"
    "SPDi23" "SPDi24" "SPDi25" "SPDi26" "SPDi27"
    "SPDi28" "SPDi29" "SPDi30" "SPDi31" "SPDi32"
    "SPDi33" "SPDi34" "SPDi35" "SPDi36" "SPDi37"
    "SPDi38" "SPDi39"
)

# Main script
echo "WIXOSS Product Batch Downloader"
echo "==============================="

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    print_error "cargo command not found. Please install Rust and cargo."
    exit 1
fi

# Parse command line arguments
case "${1:-all}" in
    booster|bo)
        print_info "Downloading booster products only..."
        download_products "booster" "${BO_CODES[@]}"
        ;;
    starter|st)
        print_info "Downloading starter products only..."
        download_products "starter" "${ST_CODES[@]}"
        ;;
    special|sp)
        print_info "Downloading special products only..."
        download_products "sp" "${SP_CODES[@]}"
        ;;
    all)
        print_info "Downloading ALL products..."
        
        total_fail=0
        
        download_products "booster" "${BO_CODES[@]}"
        ((total_fail+=$?))
        
        echo ""
        print_warning "Waiting 10 seconds before moving to starter products..."
        sleep 10
        
        download_products "starter" "${ST_CODES[@]}"
        ((total_fail+=$?))
        
        echo ""
        print_warning "Waiting 10 seconds before moving to special products..."
        sleep 10
        
        download_products "sp" "${SP_CODES[@]}"
        ((total_fail+=$?))
        
        echo ""
        echo "========================================"
        print_info "ALL DOWNLOADS COMPLETE!"
        print_info "Total products: $((${#BO_CODES[@]} + ${#ST_CODES[@]} + ${#SP_CODES[@]}))"
        if [ ${total_fail} -gt 0 ]; then
            print_error "Total failures: ${total_fail}"
        else
            print_success "All downloads successful!"
        fi
        ;;
    *)
        print_error "Invalid argument: $1"
        echo ""
        echo "Usage: $0 [booster|starter|sp|all]"
        echo ""
        echo "Options:"
        echo "  booster, bo  - Download booster products only"
        echo "  starter, st  - Download starter products only"
        echo "  special, sp  - Download special products only"
        echo "  all          - Download all products (default)"
        exit 1
        ;;
esac