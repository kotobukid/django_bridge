"""
Django Bridge専用設定
- 一時的なDjango開発サーバー起動用の設定値を管理
- Axumプロジェクトからの呼び出し用にカスタマイズ
"""

# CSRF信頼ホスト設定（Axumサーバーとの連携用）
TRUSTED_ORIGINS_BASE = [
    "http://localhost:3000",
    "http://127.0.0.1:3000",
    "http://localhost",
    "http://127.0.0.1",
    "http://localhost:3001",
    "http://127.0.0.1:3001",
]

# 管理画面専用設定
ADMIN_ONLY_APPS = [
    'django.contrib.admin',
    'django.contrib.auth',
    'django.contrib.contenttypes',
    'django.contrib.sessions',
    'django.contrib.messages',
    'django.contrib.staticfiles',
]

# プロジェクト固有アプリ
PROJECT_APPS = [
    'admin_server.apps.AdminServerConfig',
    'wix.apps.WixConfig'
]

# セキュリティ設定（開発用）
DEVELOPMENT_SECURITY = {
    'DEBUG': False,  # 管理画面専用なのでFalse
    'ALLOWED_HOSTS': ["127.0.0.1", "localhost"],
}

# 国際化設定
LOCALE_SETTINGS = {
    'LANGUAGE_CODE': 'ja',  # 日本語に変更
    'TIME_ZONE': 'Asia/Tokyo',  # 日本時間に変更
    'USE_I18N': True,
    'USE_TZ': True,
}