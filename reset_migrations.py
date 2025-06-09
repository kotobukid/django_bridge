#!/usr/bin/env python3
"""
マイグレーション状態をリセットするスクリプト
wix_rawcardテーブルがドロップされた後に実行
"""

import os
import sys
import django
from pathlib import Path

# Djangoプロジェクトのパスを追加
project_path = Path(__file__).parent / "table_definition"
sys.path.append(str(project_path))

# 環境変数設定
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')

# Django初期化
django.setup()

from django.core.management import execute_from_command_line
from django.db import connection

def reset_wix_migrations():
    """wixアプリのマイグレーション記録をリセット"""
    
    print("=== マイグレーション状態リセット開始 ===")
    
    # 1. 現在のマイグレーション状態を確認
    print("\n1. 現在のマイグレーション状態:")
    execute_from_command_line(['manage.py', 'showmigrations', 'wix'])
    
    # 2. wixアプリのマイグレーション記録を削除
    print("\n2. wixアプリのマイグレーション記録を削除中...")
    with connection.cursor() as cursor:
        cursor.execute("DELETE FROM django_migrations WHERE app = 'wix';")
        deleted_count = cursor.rowcount
        print(f"   削除されたマイグレーション記録: {deleted_count}件")
    
    # 3. マイグレーション状態を再確認
    print("\n3. リセット後のマイグレーション状態:")
    execute_from_command_line(['manage.py', 'showmigrations', 'wix'])
    
    # 4. 初期マイグレーションを作成
    print("\n4. 初期マイグレーションを作成中...")
    execute_from_command_line(['manage.py', 'makemigrations', 'wix'])
    
    # 5. マイグレーションを適用（偽適用）
    print("\n5. マイグレーションを偽適用中...")
    execute_from_command_line(['manage.py', 'migrate', 'wix', '--fake'])
    
    # 6. 最終状態確認
    print("\n6. 最終マイグレーション状態:")
    execute_from_command_line(['manage.py', 'showmigrations', 'wix'])
    
    print("\n=== マイグレーション状態リセット完了 ===")
    print("これで新しいマイグレーションを安全に適用できます。")

if __name__ == "__main__":
    reset_wix_migrations()