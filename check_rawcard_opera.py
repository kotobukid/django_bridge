#!/usr/bin/env python3
import os
import sys
import django

# Django設定
sys.path.append('/home/kotobukid/projects/wxdb/table_definition')
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
django.setup()

from wix.models import RawCard

# オペラのRawCardを検索
opera_rawcards = RawCard.objects.filter(name__icontains='オペラ')
print("オペラを含むRawCard:")
for rawcard in opera_rawcards:
    print(f"ID: {rawcard.id}, Card Number: {rawcard.card_number}, Name: {rawcard.name}")
    # HTMLの一部を表示（カード種別を含む部分）
    html = rawcard.raw_html
    if 'アーツ' in html:
        print("  HTML contains 'アーツ' ✓")
    else:
        print("  HTML does NOT contain 'アーツ' ❌")
    
    if 'ルリグ' in html:
        print("  HTML contains 'ルリグ' ❌")
    else:
        print("  HTML does NOT contain 'ルリグ' ✓")
    
    # HTMLからカード種別部分を抽出
    if '<dt>種類</dt>' in html:
        start = html.find('<dt>種類</dt>')
        end = html.find('</dd>', start) + 5
        card_type_section = html[start:end]
        print(f"  Card type section: {card_type_section}")
    print("---")