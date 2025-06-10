#!/bin/bash
# Download all スターター (Starter) products
# Usage: bash download_starter_products.sh

ST_CODES=(
    "WX25-CD1"
    "WX24-D5"
    "WX24-D4"
    "WX24-D3"
    "WX24-D2"
    "WX24-D1"
    "WXDi-D09"
    "WXDi-D08"
    "WXDi-D07"
    "WXDi-D06"
    "WXDi-D05"
    "WXDi-D04"
    "WXDi-D03"
    "WXDi-D02"
    "WXDi-D01"
    "WDA-F05"
    "WDA-F04"
    "WDA-F03"
    "WDA-F02"
    "WDA-F01"
    "WDK-17"
    "WDK-16"
    "WDK-15"
    "WDK-14"
    "WDK-13"
    "WDK-12"
    "WDK-F05"
    "WDK-F04"
    "WDK-F03"
    "WDK-F02"
    "WDK-F01"
    "WDK-11"
    "WDK-10"
    "WDK-09"
    "WDK-08"
    "WDK-07"
    "WDK-06"
    "WDK-05"
    "WDK-04"
    "WDK-03"
    "WDK-02"
    "WDK-01"
    "WXD-23"
    "WXD-22"
    "WXD-21"
    "WXD-20"
    "WXD-19"
    "WXD-18"
    "WXD-17"
    "WXD-16"
    "WXD-15"
    "WXD-14"
    "WXD-13"
    "WXD-12"
    "WXD-11"
    "WXD-10"
    "WXD-09"
    "WXD-08"
    "WXD-07"
    "WXD-06"
    "WXD-05"
    "WXD-04"
    "WXD-03"
    "WXD-02"
    "WXD-01"
)

echo "Starting download of ${#ST_CODES[@]} starter products..."
echo "=================================================="

success_count=0
fail_count=0

for code in "${ST_CODES[@]}"; do
    echo ""
    echo "Downloading ${code}..."
    cargo run --release -p wxdb-scraper -- starter "${code}"
    
    if [ $? -eq 0 ]; then
        echo "✓ Successfully downloaded ${code}"
        ((success_count++))
    else
        echo "✗ Failed to download ${code}"
        ((fail_count++))
    fi
    
    # Wait between downloads to avoid overwhelming the server
    if [ "${code}" != "${ST_CODES[-1]}" ]; then
        echo "Waiting 5 seconds before next download..."
        sleep 5
    fi
done

echo ""
echo "=================================================="
echo "Starter downloads complete!"
echo "Success: ${success_count}"
echo "Failed: ${fail_count}"
echo "Total: ${#ST_CODES[@]}"