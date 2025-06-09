#!/usr/bin/env python3
import os
import sys
import django

# Django設定
sys.path.append('/home/kotobukid/projects/wxdb/table_definition')
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
django.setup()

from wix.models import RawCard, Card

# オペラのRawCardを一つ選んで解析フラグをリセット
opera_rawcard = RawCard.objects.filter(name__icontains='オペラ').first()
if opera_rawcard:
    print(f"Selected RawCard: {opera_rawcard.id} - {opera_rawcard.name}")
    
    # 解析フラグをリセット
    opera_rawcard.is_analyzed = False
    opera_rawcard.save()
    print("解析フラグをリセットしました。")
    
    # 既存のCardエントリも確認
    existing_card = Card.objects.filter(code=opera_rawcard.card_number).first()
    if existing_card:
        print(f"既存のCard: {existing_card.id} - CardType: {existing_card.card_type}")
else:
    print("オペラのRawCardが見つかりませんでした。")