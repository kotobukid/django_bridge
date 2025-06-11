#!/usr/bin/env python
"""
現在のカードデータに含まれる商品IDとその商品情報を確認するスクリプト
"""
import os
import sys
import django

# Django settings setup
sys.path.append(os.path.join(os.path.dirname(__file__), '../table_definition'))
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
django.setup()

from wix.models import Card, Product
from collections import Counter

def main():
    # カードの商品IDを集計
    product_ids = Card.objects.values_list('product', flat=True)
    product_counter = Counter(product_ids)
    
    print("=== カードデータに含まれる商品一覧 ===")
    print(f"総カード数: {len(product_ids)}")
    print(f"商品種類数: {len(product_counter)}")
    print()
    
    # 商品情報を取得して表示
    for product_id, count in sorted(product_counter.items()):
        if product_id:
            try:
                product = Product.objects.get(id=product_id)
                print(f"ID: {product_id:3d} | Code: {product.product_code:12s} | Name: {product.name:40s} | Cards: {count:4d}")
            except Product.DoesNotExist:
                print(f"ID: {product_id:3d} | (Product not found) | Cards: {count:4d}")
        else:
            print(f"ID: None | (No product assigned) | Cards: {count:4d}")

if __name__ == "__main__":
    main()