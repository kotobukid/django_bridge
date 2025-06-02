"""
WIXOSS管理画面設定

Django Bridgeプロジェクト用の管理画面カスタマイズ
- WIXOSSカードゲーム関連のモデルを管理画面に登録
- Axumプロジェクトからアクセス可能な管理インターフェースを提供
"""
from django.contrib import admin
from .models import Card, CardType, Color, Lrig, Product, Klass, Feature, Timing


# 管理画面のサイト設定
admin.site.site_header = "WIXOSS Card Database"
admin.site.site_title = "WIXOSS Admin"
admin.site.index_title = "WIXOSS カードデータベース管理"


@admin.register(Product)
class ProductAdmin(admin.ModelAdmin):
    list_display = ('product_code', 'name', 'product_type', 'sort_asc')
    list_filter = ('product_type',)
    search_fields = ('product_code', 'name')
    ordering = ('sort_asc', 'product_code')


@admin.register(Card)
class CardAdmin(admin.ModelAdmin):
    list_display = ('code', 'name', 'get_product_name', 'get_card_type_name', 'color', 'level', 'cost')
    list_filter = ('product', 'card_type', 'color', 'level', 'has_burst', 'format')
    search_fields = ('code', 'name', 'skill_text')
    ordering = ('product', 'code')
    filter_horizontal = ('user', 'klass', 'feature')
    
    def get_product_name(self, obj):
        try:
            product = Product.objects.get(id=obj.product)
            return product.name
        except Product.DoesNotExist:
            return f"Product ID: {obj.product}"
    get_product_name.short_description = 'Product'
    
    def get_card_type_name(self, obj):
        try:
            card_type = CardType.objects.get(id=obj.card_type)
            return card_type.name
        except CardType.DoesNotExist:
            return f"CardType ID: {obj.card_type}"
    get_card_type_name.short_description = 'Card Type'


@admin.register(CardType)
class CardTypeAdmin(admin.ModelAdmin):
    list_display = ('name', 'id')
    search_fields = ('name',)
    ordering = ('id',)


@admin.register(Color)
class ColorAdmin(admin.ModelAdmin):
    list_display = ('name', 'id')
    ordering = ('id',)


@admin.register(Lrig)
class LrigAdmin(admin.ModelAdmin):
    list_display = ('name', 'id')
    search_fields = ('name',)
    ordering = ('id',)


@admin.register(Klass)
class KlassAdmin(admin.ModelAdmin):
    list_display = ('__str__', 'cat1', 'cat2', 'cat3', 'sort_asc')
    search_fields = ('cat1', 'cat2', 'cat3')
    ordering = ('sort_asc', 'cat1')


@admin.register(Feature)
class FeatureAdmin(admin.ModelAdmin):
    list_display = ('name', 'code', 'sort_asc')
    search_fields = ('name', 'code')
    ordering = ('sort_asc', 'name')


@admin.register(Timing)
class TimingAdmin(admin.ModelAdmin):
    list_display = ('name', 'id')
    search_fields = ('name',)
    ordering = ('id',)