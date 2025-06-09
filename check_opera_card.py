#!/usr/bin/env python3
import os
import sys
import django

# Django設定
sys.path.append('/home/kotobukid/projects/wxdb/table_definition')
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
django.setup()

from wix.models import Card

# オペラのカードを検索
opera_cards = Card.objects.filter(name__icontains='オペラ')
print("オペラを含むカード:")
for card in opera_cards:
    print(f"ID: {card.id}, Code: {card.code}, Name: {card.name}, CardType: {card.card_type}")

# 特定のカード番号でも検索
card_0068 = Card.objects.filter(code='0068').first()
if card_0068:
    print(f"\nカード0068の詳細:")
    print(f"Name: {card_0068.name}")
    print(f"CardType: {card_0068.card_type}")
    print(f"CardType Name: {card_0068.card_type.name if card_0068.card_type else 'None'}")