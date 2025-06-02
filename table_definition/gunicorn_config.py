"""
Django Bridge - Gunicorn設定

一時的なDjango管理画面用のGunicorn設定
"""
import os

# 基本設定
bind = f"127.0.0.1:{os.environ.get('DJANGO_ADMIN_PORT', '8003')}"
workers = 1  # 管理画面のみなので1ワーカーで十分
timeout = 30
keepalive = 2
max_requests = 1000
max_requests_jitter = 100

# ログ設定
loglevel = "info"
accesslog = "-"  # stdout
errorlog = "-"   # stderr
access_log_format = '%h "%r" %s %b "%{Referer}i" "%{User-agent}i" %D'

# プロセス設定
daemon = False
pidfile = None
user = None
group = None
tmp_upload_dir = None

# セキュリティ
limit_request_line = 4094
limit_request_fields = 100
limit_request_field_size = 8190

# Django Bridge固有の設定
wsgi_module = "django_bridge_wsgi:application"

def on_starting(server):
    """サーバー起動時の処理"""
    server.log.info("Django Bridge Admin Server starting...")

def on_exit(server):
    """サーバー終了時の処理"""
    server.log.info("Django Bridge Admin Server shutting down...")