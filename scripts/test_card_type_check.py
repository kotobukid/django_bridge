#!/usr/bin/env python3
import os
import sys

# プロジェクトルートからの相対パスでDjango設定
project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
table_definition_path = os.path.join(project_root, 'table_definition')
sys.path.append(table_definition_path)

os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')

import django
django.setup()

from wix.models import Card
import collections

# 最新の100件のcard_typeを確認
recent_cards = Card.objects.order_by('-id')[:100]
card_types = [card.card_type for card in recent_cards]
card_type_counts = collections.Counter(card_types)

print("Recent card type distribution (last 100 cards):")
for card_type, count in card_type_counts.items():
    type_name = {
        0: "Unknown",
        1: "Lrig", 2: "LrigAssist", 3: "Arts", 4: "Key",
        5: "Signi", 6: "Spell", 7: "Resona", 8: "SigniCraft",
        9: "ArtsCraft", 10: "ResonaCraft", 11: "SpellCraft",
        12: "Piece", 13: "PieceRelay", 14: "PieceCraft", 15: "Token"
    }.get(card_type, f"Unknown({card_type})")
    print(f"  {type_name}: {count} cards")

# 特定のcard_typeの例を表示
print("\nExamples by card type:")
for card_type in sorted(card_type_counts.keys()):
    example = recent_cards.filter(card_type=card_type).first()
    if example:
        type_name = {
            0: "Unknown", 1: "Lrig", 2: "LrigAssist", 3: "Arts", 4: "Key",
            5: "Signi", 6: "Spell", 7: "Resona", 8: "SigniCraft",
            9: "ArtsCraft", 10: "ResonaCraft", 11: "SpellCraft",
            12: "Piece", 13: "PieceRelay", 14: "PieceCraft", 15: "Token"
        }.get(card_type, f"Unknown({card_type})")
        print(f"  {type_name}: {example.name} (ID: {example.id})")