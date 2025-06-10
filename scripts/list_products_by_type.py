#!/usr/bin/env python3
"""
List all product codes from the database grouped by type.
This script extracts product codes to help create batch download scripts.
"""
import os
import sys
import django

# プロジェクトルートからの相対パスでDjango設定
project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
table_definition_path = os.path.join(project_root, 'table_definition')
sys.path.append(table_definition_path)

os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
django.setup()

from wix.models import Product
from collections import defaultdict

def list_products_by_type():
    """List all products grouped by type with their codes."""
    
    # Group products by type
    products_by_type = defaultdict(list)
    
    for product in Product.objects.all().order_by('sort_asc'):
        products_by_type[product.product_type].append({
            'code': product.product_code,
            'name': product.name,
            'sort': product.sort_asc
        })
    
    # Print results
    print("Products grouped by type:")
    print("=" * 80)
    
    type_names = {
        'bo': 'ブースター (Booster)',
        'st': 'スターター (Starter)',
        'pr': 'プロモーション (Promotion)',
        'sp': 'スペシャル (Special)'
    }
    
    for product_type, products in sorted(products_by_type.items()):
        type_name = type_names.get(product_type, product_type)
        print(f"\n{type_name} ({product_type}) - {len(products)} products:")
        print("-" * 60)
        
        for product in products:
            print(f"  {product['code']:<15} {product['name']}")
        
        # Also print just the codes for easy copying
        print(f"\nCodes only for {product_type}:")
        codes = [p['code'] for p in products]
        print(" ".join(codes))
        print()

    # Create separate lists for batch processing
    print("\n" + "=" * 80)
    print("Ready-to-use lists for batch processing:")
    print("=" * 80)
    
    for product_type, products in sorted(products_by_type.items()):
        type_name = type_names.get(product_type, product_type)
        print(f"\n# {type_name} ({product_type})")
        codes = [p['code'] for p in products]
        
        # Print as Python list
        print(f"{product_type.upper()}_CODES = [")
        for i, code in enumerate(codes):
            if i % 5 == 0:
                print("    ", end="")
            print(f'"{code}"', end="")
            if i < len(codes) - 1:
                print(", ", end="")
            if (i + 1) % 5 == 0 and i < len(codes) - 1:
                print()
        print("\n]")
        
        # Print as bash array
        print(f"\n# Bash array for {product_type}")
        print(f'{product_type.upper()}_CODES=(')
        for code in codes:
            print(f'    "{code}"')
        print(')')

if __name__ == "__main__":
    list_products_by_type()