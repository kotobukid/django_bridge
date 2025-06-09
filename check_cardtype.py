#!/usr/bin/env python3
import os
import sys
import django

# Django設定
sys.path.append('/home/kotobukid/projects/wxdb/table_definition')
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
django.setup()

from wix.models import CardType

print('CardType mappings:')
for ct in CardType.objects.all().order_by('id'):
    print(f'ID: {ct.id}, Name: {ct.name}, Sort: {ct.sort_asc}')