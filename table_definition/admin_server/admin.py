"""
Django Bridge - Admin Server Admin Configuration

admin_serverアプリの管理画面設定
"""
from django.contrib import admin
from .models import SiteSetting


@admin.register(SiteSetting)
class SiteSettingAdmin(admin.ModelAdmin):
    """SiteSetting管理画面設定"""
    
    list_display = ('key', 'value_preview', 'description', 'updated_at')
    list_filter = ('created_at', 'updated_at')
    search_fields = ('key', 'description', 'value')
    ordering = ('key',)
    
    fields = ('key', 'value', 'description')
    
    def value_preview(self, obj):
        """値のプレビュー表示（長い場合は省略）"""
        if len(obj.value) > 50:
            return f"{obj.value[:50]}..."
        return obj.value
    value_preview.short_description = "設定値"
    
    def get_readonly_fields(self, request, obj=None):
        """編集時はkeyを読み取り専用にする"""
        if obj:  # 編集時
            return ('key',)
        return ()
    
    def has_delete_permission(self, request, obj=None):
        """重要な設定の削除を制限"""
        if obj and obj.key in ['admin_root']:
            return False
        return super().has_delete_permission(request, obj)