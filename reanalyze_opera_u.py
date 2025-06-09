#!/usr/bin/env python3
import os
import sys
import django

# Django設定
sys.path.append('/home/kotobukid/projects/wxdb/table_definition')
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
django.setup()

from wix.models import RawCard

# もう一つのオペラカード（U版）を処理
opera_rawcard = RawCard.objects.filter(card_number='WX25-CP1-008U').first()
if opera_rawcard:
    print(f"Selected RawCard: {opera_rawcard.id} - {opera_rawcard.name}")
    
    # 解析フラグをリセット
    opera_rawcard.is_analyzed = False
    opera_rawcard.save()
    print("解析フラグをリセットしました。")
else:
    print("オペラU版のRawCardが見つかりませんでした。")