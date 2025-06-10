#!/bin/bash
# Download all ブースター (Booster) products
# Usage: bash download_booster_products.sh

BO_CODES=(
    "WX25-CP1"
    "WX24-P4"
    "WX24-P3"
    "WX24-P2"
    "WX24-P1"
    "WXDi-P16"
    "WXDi-P15"
    "WXDi-CP02"
    "WXDi-P14"
    "WXDi-P13"
    "WXDi-P12"
    "WXDi-CP01"
    "WXDi-P11"
    "WXDi-P10"
    "WXDi-P09"
    "WXDi-P08"
    "WXDi-P07"
    "WXDi-P06"
    "WXDi-P05"
    "WXDi-P04"
    "WXDi-P03"
    "WXDi-P02"
    "WXDi-P01"
    "WXDi-P00"
    "WXK-11"
    "WXK-10"
    "WXK-09"
    "WXK-08"
    "WXK-07"
    "WXK-06"
    "WXK-05"
    "WXK-04"
    "WXK-03"
    "WXK-02"
    "WXK-01"
    "WXEX-2"
    "WXEX-1"
    "WX-22"
    "WX-21"
    "WX-20"
    "WX-19"
    "WX-18"
    "WX-17"
    "WX-16"
    "WX-15"
    "WX-14"
    "WX-13"
    "WX-12"
    "WX-11"
    "WX-10"
    "WX-09"
    "WX-08"
    "WX-07"
    "WX-06"
    "WX-05"
    "WX-04"
    "WX-03"
    "WX-02"
    "WX-01"
)

echo "Starting download of ${#BO_CODES[@]} booster products..."
echo "=================================================="

success_count=0
fail_count=0

for code in "${BO_CODES[@]}"; do
    echo ""
    echo "Downloading ${code}..."
    cargo run --release -p wxdb-scraper -- booster "${code}"
    
    if [ $? -eq 0 ]; then
        echo "✓ Successfully downloaded ${code}"
        ((success_count++))
    else
        echo "✗ Failed to download ${code}"
        ((fail_count++))
    fi
    
    # Wait between downloads to avoid overwhelming the server
    if [ "${code}" != "${BO_CODES[-1]}" ]; then
        echo "Waiting 5 seconds before next download..."
        sleep 5
    fi
done

echo ""
echo "=================================================="
echo "Booster downloads complete!"
echo "Success: ${success_count}"
echo "Failed: ${fail_count}"
echo "Total: ${#BO_CODES[@]}"