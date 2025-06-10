#!/bin/bash
# Download all スペシャル (Special) products
# Usage: bash download_special_products.sh

SP_CODES=(
    "SPDi01"
    "SPDi02"
    "SPDi03"
    "SPDi04"
    "SPDi05"
    "SPDi06"
    "SPDi07"
    "SPDi08"
    "SPDi09"
    "SPDi10"
    "SPDi11"
    "SPDi12"
    "SPDi13"
    "SPDi14"
    "SPDi15"
    "SPDi16"
    "SPDi17"
    "SPDi19"
    "SPDi20"
    "SPDi21"
    "SPDi23"
    "SPDi24"
    "SPDi25"
    "SPDi26"
    "SPDi27"
    "SPDi28"
    "SPDi29"
    "SPDi30"
    "SPDi31"
    "SPDi32"
    "SPDi33"
    "SPDi34"
    "SPDi35"
    "SPDi36"
    "SPDi37"
    "SPDi38"
    "SPDi39"
)

echo "Starting download of ${#SP_CODES[@]} special products..."
echo "=================================================="

success_count=0
fail_count=0

for code in "${SP_CODES[@]}"; do
    echo ""
    echo "Downloading ${code}..."
    cargo run --release -p wxdb-scraper -- sp "${code}"
    
    if [ $? -eq 0 ]; then
        echo "✓ Successfully downloaded ${code}"
        ((success_count++))
    else
        echo "✗ Failed to download ${code}"
        ((fail_count++))
    fi
    
    # Wait between downloads to avoid overwhelming the server
    if [ "${code}" != "${SP_CODES[-1]}" ]; then
        echo "Waiting 5 seconds before next download..."
        sleep 5
    fi
done

echo ""
echo "=================================================="
echo "Special downloads complete!"
echo "Success: ${success_count}"
echo "Failed: ${fail_count}"
echo "Total: ${#SP_CODES[@]}"