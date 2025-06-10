#!/usr/bin/env python3
"""
製品のスクレイピング・解析状況をチェックするスクリプト
"""

import os
import sys
import django

# Django設定
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'table_definition'))
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
django.setup()

from wix.models import Product, RawCard, Card
from django.db.models import Count

def main():
    print("=== WIXOSS Product Status Check ===\n")
    
    # 製品タイプごとの統計
    product_types = {
        'bo': 'Booster',
        'st': 'Starter',
        'sp': 'Special',
        'pr': 'Promotion'
    }
    
    for ptype, pname in product_types.items():
        products = Product.objects.filter(product_type=ptype).order_by('product_code')
        
        print(f"\n--- {pname} Products ({products.count()}) ---")
        
        not_scraped = []
        scraped_not_analyzed = []
        fully_processed = []
        
        for product in products:
            raw_count = RawCard.objects.filter(product=product).count()
            card_count = Card.objects.filter(product=product).count()
            
            if raw_count == 0:
                not_scraped.append(product.product_code)
            elif card_count == 0:
                scraped_not_analyzed.append((product.product_code, raw_count))
            else:
                fully_processed.append((product.product_code, raw_count, card_count))
        
        # 未スクレイピング
        if not_scraped:
            print(f"\n  📋 Not scraped ({len(not_scraped)}):")
            for code in not_scraped:
                print(f"    - {code}")
        
        # スクレイピング済み・未解析
        if scraped_not_analyzed:
            print(f"\n  ⚠️  Scraped but not analyzed ({len(scraped_not_analyzed)}):")
            for code, raw_count in scraped_not_analyzed:
                print(f"    - {code}: {raw_count} raw cards")
        
        # 完全処理済み
        if fully_processed:
            print(f"\n  ✅ Fully processed ({len(fully_processed)}):")
            for code, raw_count, card_count in fully_processed[:5]:  # 最初の5件のみ表示
                print(f"    - {code}: {raw_count} raw → {card_count} cards")
            if len(fully_processed) > 5:
                print(f"    ... and {len(fully_processed) - 5} more")
    
    # 全体統計
    print("\n\n=== Overall Statistics ===")
    total_products = Product.objects.count()
    total_raw_cards = RawCard.objects.count()
    total_cards = Card.objects.count()
    
    # 製品ごとのRawCard数
    products_with_raw = Product.objects.annotate(
        raw_count=Count('rawcard')
    ).filter(raw_count__gt=0).count()
    
    # 製品ごとのCard数
    products_with_cards = Product.objects.annotate(
        card_count=Count('card')
    ).filter(card_count__gt=0).count()
    
    print(f"Total products: {total_products}")
    print(f"Products with raw cards: {products_with_raw} ({products_with_raw/total_products*100:.1f}%)")
    print(f"Products with analyzed cards: {products_with_cards} ({products_with_cards/total_products*100:.1f}%)")
    print(f"Total raw cards: {total_raw_cards:,}")
    print(f"Total analyzed cards: {total_cards:,}")
    
    # 解析待ちのRawCard
    analyzed_raw_ids = Card.objects.values_list('raw_card_id', flat=True)
    unanalyzed_count = RawCard.objects.exclude(id__in=analyzed_raw_ids).count()
    
    if unanalyzed_count > 0:
        print(f"\n⚠️  Unanalyzed raw cards: {unanalyzed_count:,}")
        print("  Run 'cargo run -p analyzer' to analyze them")

if __name__ == "__main__":
    main()