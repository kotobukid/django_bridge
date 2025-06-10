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

from wix.models import CardType

print('CardType mappings:')
for ct in CardType.objects.all().order_by('id'):
    print(f'ID: {ct.id}, Name: {ct.name}, Sort: {ct.sort_asc}')