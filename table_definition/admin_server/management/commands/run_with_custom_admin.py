"""
Django Bridge - カスタム管理画面起動コマンド

Axumプロジェクトから動的にDjango管理画面を起動するための
カスタムrunserverコマンドです。

主な機能：
1. 動的な管理画面URL設定（--admin-root）
2. 動的なCSRF信頼ポート設定（--csrf-trust-port）
3. Axumサーバーとの連携用設定の注入
"""
from django.core.management.commands.runserver import Command as BaseRunserverCommand
from django.conf import settings
import sys


class Command(BaseRunserverCommand):
    """
    Django Bridge専用のrunserverコマンド
    
    通常のrunserverコマンドを拡張し、以下の機能を追加：
    - カスタム管理画面URL設定
    - 動的CSRF信頼ポート設定
    """
    
    help = (
        "Django Bridge用の開発サーバーを起動します。"
        "Axumプロジェクトからの動的設定に対応しています。"
    )
    
    def add_arguments(self, parser):
        """コマンドライン引数の追加"""
        super().add_arguments(parser)
        
        parser.add_argument(
            '--admin-root',
            type=str,
            default='admin/',
            help='管理画面のカスタムURL root（例: "custom_admin/"）',
        )
        
        parser.add_argument(
            '--csrf-trust-port',
            type=str,
            help='CSRF信頼ポート番号（Axumサーバーとの連携用）',
        )

    def execute(self, *args, **options):
        """コマンド実行時の処理"""
        self._setup_admin_root(options)
        self._setup_csrf_trust_port(options)
        
        # Django開発サーバーを起動
        super().execute(*args, **options)
    
    def _setup_admin_root(self, options):
        """管理画面URLの動的設定"""
        admin_root = options.get('admin_root', 'admin/')
        
        # URLの末尾にスラッシュを確保
        if admin_root and not admin_root.endswith('/'):
            admin_root += '/'
        
        settings.CUSTOM_ADMIN_ROOT = admin_root
        
        self.stdout.write(
            self.style.SUCCESS(f'✓ Admin root set to: {admin_root}')
        )
    
    def _setup_csrf_trust_port(self, options):
        """CSRF信頼ポートの動的設定"""
        csrf_trust_port = options.get('csrf_trust_port')
        
        if csrf_trust_port:
            trust_origins = [
                f"http://localhost:{csrf_trust_port}",
                f"http://127.0.0.1:{csrf_trust_port}",
            ]
            
            settings.CSRF_TRUSTED_ORIGINS.extend(trust_origins)
            
            self.stdout.write(
                self.style.SUCCESS(f'✓ CSRF trust port added: {csrf_trust_port}')
            )
    
    def inner_run(self, *args, **options):
        """サーバー起動時の追加情報表示"""
        self.stdout.write("")
        self.stdout.write(self.style.HTTP_INFO("Django Bridge Admin Server"))
        self.stdout.write(self.style.HTTP_INFO("=" * 50))
        self.stdout.write(f"Admin URL: {settings.CUSTOM_ADMIN_ROOT}")
        self.stdout.write(f"Health Check: {settings.CUSTOM_ADMIN_ROOT}health-check")
        self.stdout.write("")
        
        # 通常のサーバー起動処理
        super().inner_run(*args, **options)
