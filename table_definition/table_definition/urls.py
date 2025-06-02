"""
Django Bridge - URL設定

特殊な用途のため、以下のURL構成になっています：
1. 動的な管理画面ルート（CUSTOM_ADMIN_ROOTで設定）
2. ヘルスチェック用エンドポイント（Axumからの監視用）
3. 静的ファイル配信（本番環境用）

通常のDjangoアプリケーションとは異なり、管理画面とヘルスチェックのみを提供します。
"""
from django.contrib import admin
from django.urls import path, re_path
from django.conf import settings
from django.views.static import serve
from admin_server.views import health_check_admin


def build_urlpatterns():
    """
    動的URL構成の構築
    CUSTOM_ADMIN_ROOTの値に応じてURL構成を変更
    """
    patterns = []
    
    # ヘルスチェックエンドポイント
    # 1. 固定パス（Axumサーバーからの監視用）
    patterns.append(path('admin_proxy/health-check', health_check_admin, name='health_check_proxy'))
    
    # 2. 動的パス（カスタム管理画面ルートに対応）
    patterns.append(path(f'{settings.CUSTOM_ADMIN_ROOT}health-check', health_check_admin, name='health_check_custom'))
    
    # 管理画面（動的ルート）
    patterns.append(path(settings.CUSTOM_ADMIN_ROOT, admin.site.urls))
    
    # 静的ファイル配信（本番環境のみ）
    if not settings.DEBUG:
        static_pattern = re_path(
            r'^%s(?P<path>.*)$' % settings.STATIC_URL.lstrip('/'), 
            serve, 
            {'document_root': settings.STATIC_ROOT},
            name='static_files'
        )
        patterns.append(static_pattern)
    
    return patterns


urlpatterns = build_urlpatterns()
