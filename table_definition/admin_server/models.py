"""
Django Bridge - Admin Server Models

サイト設定や管理用データを保存するためのモデル定義
"""
from django.db import models


class SiteSetting(models.Model):
    """
    汎用的なサイト設定を保存するモデル
    
    Key-Value形式で様々な設定を保存できる。
    例：admin_root, maintenance_mode, など
    """
    
    key = models.CharField(
        verbose_name="設定キー",
        max_length=100,
        unique=True,
        help_text="設定の識別キー（例: admin_root, maintenance_mode）"
    )
    
    value = models.TextField(
        verbose_name="設定値",
        blank=True,
        help_text="設定の値（文字列、JSON等）"
    )
    
    description = models.CharField(
        verbose_name="説明",
        max_length=255,
        blank=True,
        help_text="この設定の説明"
    )
    
    created_at = models.DateTimeField(
        verbose_name="作成日時",
        auto_now_add=True
    )
    
    updated_at = models.DateTimeField(
        verbose_name="更新日時",
        auto_now=True
    )
    
    class Meta:
        verbose_name = "サイト設定"
        verbose_name_plural = "サイト設定"
        ordering = ['key']
    
    def __str__(self):
        return f"{self.key}: {self.value[:50]}{'...' if len(self.value) > 50 else ''}"
    
    @classmethod
    def get_value(cls, key: str, default=None):
        """
        設定値を取得するヘルパーメソッド
        
        Args:
            key: 設定キー
            default: デフォルト値
            
        Returns:
            設定値またはデフォルト値
        """
        try:
            setting = cls.objects.get(key=key)
            return setting.value
        except cls.DoesNotExist:
            return default
    
    @classmethod
    def set_value(cls, key: str, value: str, description: str = ""):
        """
        設定値を保存するヘルパーメソッド
        
        Args:
            key: 設定キー
            value: 設定値
            description: 設定の説明
        """
        setting, created = cls.objects.get_or_create(
            key=key,
            defaults={
                'value': value,
                'description': description
            }
        )
        
        if not created:
            setting.value = value
            if description:
                setting.description = description
            setting.save()
        
        return setting