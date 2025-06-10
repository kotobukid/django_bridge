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

from wix.models import Card, CardType
from django.db.models import Count

print("現在のカードタイプ分布:")
print("=" * 50)

# 現在のカードタイプ分布を取得
card_type_counts = Card.objects.values('card_type__name', 'card_type__id').annotate(count=Count('id')).order_by('card_type__id')

for item in card_type_counts:
    card_type_name = item['card_type__name'] or "NULL"
    card_type_id = item['card_type__id'] or "NULL"
    count = item['count']
    print(f"ID {card_type_id}: {card_type_name} - {count} cards")

print(f"\n総カード数: {Card.objects.count()}")

# 特定のカードタイプをサンプル表示
print("\n各カードタイプのサンプル:")
print("=" * 30)

for card_type in CardType.objects.all().order_by('id')[:5]:  # 最初の5つのタイプ
    sample_cards = Card.objects.filter(card_type=card_type)[:3]  # 各タイプから3枚
    if sample_cards:
        print(f"\n{card_type.name} (ID: {card_type.id}):")
        for card in sample_cards:
            print(f"  - {card.code}: {card.name[:50]}...")