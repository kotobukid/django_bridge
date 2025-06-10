#!/usr/bin/env python3
import os
import sys
import django

# プロジェクトルートからの相対パスでDjango設定
project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
table_definition_path = os.path.join(project_root, 'table_definition')
sys.path.append(table_definition_path)

os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
django.setup()

from wix.models import Card, RawCard

# キーとして検出されたカードを調査
key_cards = Card.objects.filter(card_type=8)  # キーのID
print(f"キーとして検出されたカード数: {key_cards.count()}")
print("=" * 60)

for card in key_cards:
    print(f"Card ID: {card.id}")
    print(f"Code: {card.code}")
    print(f"Name: {card.name}")
    
    # 対応するRawCardを検索
    raw_card = RawCard.objects.filter(card_number=card.code).first()
    if raw_card:
        html = raw_card.raw_html
        print(f"RawCard ID: {raw_card.id}")
        
        # HTMLの中で「キー」が出現する箇所を特定
        if 'キー' in html:
            # キーが含まれる前後の文脈を表示
            key_positions = []
            start = 0
            while True:
                pos = html.find('キー', start)
                if pos == -1:
                    break
                key_positions.append(pos)
                start = pos + 1
            
            print(f"「キー」が見つかった位置数: {len(key_positions)}")
            for i, pos in enumerate(key_positions):
                context_start = max(0, pos - 50)
                context_end = min(len(html), pos + 50)
                context = html[context_start:context_end]
                print(f"  位置 {i+1}: ...{context}...")
        
        # カード種類セクションを確認
        if '<dt>カード種類</dt>' in html:
            start = html.find('<dt>カード種類</dt>')
            after_dt = html[start:]
            if '<dd>' in after_dt and '</dd>' in after_dt:
                dd_start = after_dt.find('<dd>') + 4
                dd_end = after_dt.find('</dd>')
                card_type_text = after_dt[dd_start:dd_end]
                print(f"カード種類セクション: '{card_type_text}'")
        
        # 旧形式のカード種類セクションも確認
        if '<dt>種類</dt>' in html:
            start = html.find('<dt>種類</dt>')
            after_dt = html[start:]
            if '<dd>' in after_dt and '</dd>' in after_dt:
                dd_start = after_dt.find('<dd>') + 4
                dd_end = after_dt.find('</dd>')
                card_type_text = after_dt[dd_start:dd_end]
                print(f"種類セクション: '{card_type_text}'")
    
    print("-" * 40)