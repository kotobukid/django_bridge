"""
Django Bridge - WSGI Configuration for Production

GunicornやuWSGIなどのWSGIサーバーで起動する場合の設定
"""
import os
import sys
from pathlib import Path

# Django Bridge固有の設定
DJANGO_BRIDGE_DIR = Path(__file__).resolve().parent

# Djangoプロジェクトのパスを追加
sys.path.insert(0, str(DJANGO_BRIDGE_DIR))
sys.path.insert(0, str(DJANGO_BRIDGE_DIR / 'table_definition'))

# Django設定モジュールを指定
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')

# Django application取得
from django.core.wsgi import get_wsgi_application
from django.conf import settings

# カスタム管理画面ルートの設定（環境変数から取得）
if os.environ.get('DJANGO_BRIDGE_ADMIN_ROOT'):
    settings.CUSTOM_ADMIN_ROOT = os.environ['DJANGO_BRIDGE_ADMIN_ROOT']

application = get_wsgi_application()