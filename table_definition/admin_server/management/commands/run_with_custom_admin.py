from django.core.management.commands.runserver import Command as BaseRunserverCommand
from django.conf import settings
import sys

class Command(BaseRunserverCommand):
    def add_arguments(self, parser):
        super().add_arguments(parser)
        # 新しい起動オプションを追加
        parser.add_argument(
            '--admin-root',
            type=str,
            help='Set a custom admin URL root for this session.',
        )
        parser.add_argument(
            '--csrf-trust-port',
            type=str,
            help='Set a custom port number for CSRF trust.',
        )

    def execute(self, *args, **options):
        # --admin-rootオプションを取得して設定に適用
        admin_root = options.get('admin_root')
        if admin_root:
            print(f"Using custom admin root: {admin_root}")
            settings.CUSTOM_ADMIN_ROOT = admin_root
        else:
            settings.CUSTOM_ADMIN_ROOT = "admin/"  # デフォルト

        csrf_trust_port = options.get('csrf_trust_port')
        if csrf_trust_port:
            print(f"Using custom CSRF trust port: {csrf_trust_port}")
            settings.CSRF_TRUSTED_ORIGINS += [
                f"http://localhost:{csrf_trust_port}",
                f"http://127.0.0.1:{csrf_trust_port}",
            ]

        # 継承元の execute を呼び出して通常のrunserverをスタート
        super().execute(*args, **options)
