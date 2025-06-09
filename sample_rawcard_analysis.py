#!/usr/bin/env python3
import os
import sys
import django

# Django設定
sys.path.append('/home/kotobukid/projects/wxdb/table_definition')
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
django.setup()

from wix.models import RawCard
import random

# ランダムに10枚のRawCardを選んでHTMLを分析
raw_cards = list(RawCard.objects.all())
sample_cards = random.sample(raw_cards, min(10, len(raw_cards)))

print("ランダムサンプルのHTMLカード種類分析:")
print("=" * 60)

for raw_card in sample_cards:
    print(f"Card: {raw_card.card_number} - {raw_card.name[:30]}...")
    html = raw_card.raw_html
    
    # カード種類セクションを探す
    card_type_found = False
    
    # 新形式
    if '<dt>カード種類</dt>' in html:
        start = html.find('<dt>カード種類</dt>')
        after_dt = html[start:]
        if '<dd>' in after_dt and '</dd>' in after_dt:
            dd_start = after_dt.find('<dd>') + 4
            dd_end = after_dt.find('</dd>')
            card_type_text = after_dt[dd_start:dd_end]
            print(f"  カード種類: '{card_type_text}'")
            card_type_found = True
    
    # 旧形式
    if not card_type_found and '<dt>種類</dt>' in html:
        start = html.find('<dt>種類</dt>')
        after_dt = html[start:]
        if '<dd>' in after_dt and '</dd>' in after_dt:
            dd_start = after_dt.find('<dd>') + 4
            dd_end = after_dt.find('</dd>')
            card_type_text = after_dt[dd_start:dd_end]
            print(f"  種類: '{card_type_text}'")
            card_type_found = True
    
    if not card_type_found:
        print("  カード種類セクションが見つからない")
        # HTMLの最初の1000文字を表示
        print(f"  HTML sample: {html[:500]}...")
    
    print("-" * 40)