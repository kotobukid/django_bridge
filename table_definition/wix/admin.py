"""
WIXOSS管理画面設定

Django Bridgeプロジェクト用の管理画面カスタマイズ
- WIXOSSカードゲーム関連のモデルを管理画面に登録
- Axumプロジェクトからアクセス可能な管理インターフェースを提供
"""
from django.contrib import admin
from .models import Card, CardType, Color, Lrig, Product, Klass, Feature, Timing, RawCard


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


@admin.register(RawCard)
class RawCardAdmin(admin.ModelAdmin):
    list_display = ('card_number', 'name', 'product', 'scraped_at', 'is_analyzed', 'has_skill', 'has_burst')
    list_filter = ('product', 'is_analyzed', 'scraped_at')
    search_fields = ('card_number', 'name', 'skill_text', 'life_burst_text')
    ordering = ('-scraped_at',)
    readonly_fields = ('scraped_at', 'raw_html_preview')
    date_hierarchy = 'scraped_at'
    
    fieldsets = (
        ('基本情報', {
            'fields': ('card_number', 'name', 'product', 'source_url')
        }),
        ('スクレイピングデータ', {
            'fields': ('raw_html_preview', 'skill_text', 'life_burst_text'),
            'classes': ('wide',)
        }),
        ('解析状態', {
            'fields': ('is_analyzed', 'last_analyzed_at', 'analysis_error'),
            'classes': ('collapse',)
        }),
        ('メタデータ', {
            'fields': ('scraped_at',),
            'classes': ('collapse',)
        }),
    )
    
    def has_skill(self, obj):
        """スキルテキストの有無を表示"""
        return bool(obj.skill_text)
    has_skill.boolean = True
    has_skill.short_description = 'スキル有'
    
    def has_burst(self, obj):
        """ライフバーストの有無を表示"""
        return bool(obj.life_burst_text)
    has_burst.boolean = True
    has_burst.short_description = 'バースト有'
    
    def raw_html_preview(self, obj):
        """生HTMLのプレビュー（最初の500文字）"""
        from django.utils.html import format_html
        if obj.raw_html:
            preview = obj.raw_html[:500] + '...' if len(obj.raw_html) > 500 else obj.raw_html
            return format_html('<pre style="white-space: pre-wrap; word-wrap: break-word;">{}</pre>', preview)
        return '-'
    raw_html_preview.short_description = '生HTML (プレビュー)'
    
    def get_queryset(self, request):
        """クエリセットの最適化"""
        return super().get_queryset(request).select_related('product')
    
    actions = ['mark_as_analyzed', 'mark_as_not_analyzed', 'reanalyze']
    
    def mark_as_analyzed(self, request, queryset):
        """選択したカードを解析済みにマーク"""
        updated = queryset.update(is_analyzed=True)
        self.message_user(request, f'{updated}件のカードを解析済みにマークしました。')
    mark_as_analyzed.short_description = '解析済みにマーク'
    
    def mark_as_not_analyzed(self, request, queryset):
        """選択したカードを未解析にマーク"""
        updated = queryset.update(is_analyzed=False, analysis_error='')
        self.message_user(request, f'{updated}件のカードを未解析にマークしました。')
    mark_as_not_analyzed.short_description = '未解析にマーク'
    
    def reanalyze(self, request, queryset):
        """選択したカードの再解析をリクエスト"""
        from django.utils import timezone
        updated = queryset.update(
            is_analyzed=False,
            analysis_error='',
            last_analyzed_at=timezone.now()
        )
        self.message_user(request, f'{updated}件のカードの再解析をリクエストしました。')
    reanalyze.short_description = '再解析をリクエスト'