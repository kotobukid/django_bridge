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
opera_rawcard = RawCard.objects.filter(name__icontains='オペラ').first()
if opera_rawcard:
    html = opera_rawcard.raw_html
    print("=== HTML Content Analysis ===")
    print(f"Card: {opera_rawcard.card_number} - {opera_rawcard.name}")
    print()
    
    # HTMLをセクションごとに分析
    print("アーツが含まれているかチェック:")
    if 'アーツ' in html:
        print("✓ 'アーツ' found in HTML")
        # アーツが含まれる部分を特定
        start = html.find('アーツ') - 50
        end = html.find('アーツ') + 50
        print(f"Context around 'アーツ': {html[max(0,start):end]}")
    else:
        print("❌ 'アーツ' NOT found in HTML")
    
    print("\nルリグが含まれているかチェック:")
    if 'ルリグ' in html:
        print("❌ 'ルリグ' found in HTML (this should not be here for Arts)")
        # ルリグが含まれる部分を特定
        start = html.find('ルリグ') - 50
        end = html.find('ルリグ') + 50
        print(f"Context around 'ルリグ': {html[max(0,start):end]}")
    else:
        print("✓ 'ルリグ' NOT found in HTML")
    
    # カード種類セクションを検索
    print("\nカード種類セクション:")
    if '<dt>種類</dt>' in html:
        start = html.find('<dt>種類</dt>')
        end = html.find('</dd>', start) + 5
        if end > start:
            card_type_section = html[start:end]
            print(f"種類セクション: {card_type_section}")
        else:
            print("種類セクションの終了タグが見つかりません")
    else:
        print("種類セクションが見つかりません")
        
    # HTMLの全体構造を確認（最初の1000文字）
    print("\n=== HTML Structure (first 1000 chars) ===")
    print(html[:1000])
    print("...")