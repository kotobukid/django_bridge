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

# カードタイプごとのカード数を手動で集計
for card_type in CardType.objects.all().order_by('id'):
    count = Card.objects.filter(card_type=card_type.id).count()
    print(f"ID {card_type.id}: {card_type.name} - {count} cards")

print(f"\n総カード数: {Card.objects.count()}")

# ID値による分布（card_typeが整数フィールドの場合）
print("\nカードタイプID別分布:")
card_type_id_counts = Card.objects.values('card_type').annotate(count=Count('id')).order_by('card_type')
for item in card_type_id_counts:
    card_type_id = item['card_type']
    count = item['count']
    # CardTypeのnameを取得
    try:
        card_type_name = CardType.objects.get(id=card_type_id).name
    except CardType.DoesNotExist:
        card_type_name = "Unknown"
    print(f"ID {card_type_id}: {card_type_name} - {count} cards")