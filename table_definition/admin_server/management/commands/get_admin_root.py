"""
Django Bridge - 管理画面ルート取得コマンド

現在設定されているCUSTOM_ADMIN_ROOTを取得するためのコマンド。
HTTP経由ではなく、コマンドライン経由でのみアクセス可能にして
セキュリティを保持。
"""
from django.core.management.base import BaseCommand
from django.conf import settings
import json
import sys


class Command(BaseCommand):
    """
    現在の管理画面ルートを取得するコマンド
    
    Axumサーバーから呼び出され、一時的に設定された
    CUSTOM_ADMIN_ROOTの値を取得する。
    """
    
    help = "現在設定されている管理画面のルートパスを取得します"
    
    def add_arguments(self, parser):
        """コマンドライン引数の追加"""
        parser.add_argument(
            '--format',
            choices=['json', 'text', 'raw'],
            default='json',
            help='出力フォーマット（json/text/raw）',
        )

    def handle(self, *args, **options):
        """コマンド実行処理"""
        try:
            # データベースから管理画面ルートを取得
            admin_root = self._get_admin_root_from_db()
            
            # データベースから取得できない場合は設定から取得
            if not admin_root:
                admin_root = getattr(settings, 'CUSTOM_ADMIN_ROOT', None)
            
            # 出力フォーマット別の処理
            format_type = options['format']
            
            if format_type == 'json':
                # JSON形式で出力（Axumからの解析用）
                result = {
                    'success': True,
                    'admin_root': admin_root,
                    'has_admin_root': admin_root is not None and admin_root != "",
                }
                self.stdout.write(json.dumps(result))
                
            elif format_type == 'text':
                # 人間が読める形式
                if admin_root:
                    self.stdout.write(f"Admin root: {admin_root}")
                else:
                    self.stdout.write("Admin root not set")
                    
            elif format_type == 'raw':
                # 生の値のみ（スクリプト処理用）
                self.stdout.write(admin_root or "")
            
        except Exception as e:
            # エラー時の処理
            if options['format'] == 'json':
                error_result = {
                    'success': False,
                    'error': str(e),
                    'admin_root': None,
                    'has_admin_root': False,
                }
                self.stdout.write(json.dumps(error_result))
            else:
                self.stderr.write(f"Error: {e}")
                sys.exit(1)
    
    def _get_admin_root_from_db(self):
        """データベースから管理画面ルートを取得"""
        try:
            from admin_server.models import SiteSetting
            return SiteSetting.get_value('admin_root')
        except Exception:
            # データベースアクセスエラーやモデルが存在しない場合
            return None