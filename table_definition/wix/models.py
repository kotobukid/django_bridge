from django.db import models


class Product(models.Model):
    name = models.CharField(verbose_name="名前", max_length=256, null=False, blank=False)
    product_code = models.CharField(verbose_name="商品コード", max_length=128, null=False, blank=False)
    url = models.URLField(verbose_name="商品ページ", null=True, blank=True)
    product_type = models.CharField(
        verbose_name="商品タイプ",
        max_length=2, null=False, blank=False,
        choices=[("bo", "ブースター"), ("st", "スターター"), ("pr", "プロモーション"), ("sp", "スペシャル")]
    )
    sort_asc = models.IntegerField(verbose_name="ソート順(昇順)", null=False, blank=False, default=0)

    def __str__(self):
        return self.name

    class Meta:
        verbose_name = "製品"
        verbose_name_plural = "製品"


class Color(models.Model):  # 白、赤…無色
    name = models.CharField(verbose_name="名前", max_length=256, null=False, blank=False)
    code = models.CharField(verbose_name="内部値", max_length=16, null=False, blank=False)
    bit = models.IntegerField(verbose_name="ビット値", null=False, blank=False, default=1 << 7)
    sort_asc = models.IntegerField(verbose_name="ソート順(昇順)", null=False, blank=False, default=0)

    def __str__(self):
        return f"{self.name} ({self.code.upper()})"

    class Meta:
        verbose_name = "色"
        verbose_name_plural = "色"


class CardType(models.Model):  # シグニ、ルリグ…
    name = models.CharField(verbose_name="名前", max_length=8, null=False, blank=False)
    code = models.CharField(verbose_name="内部値", max_length=32, null=False, blank=False)
    sort_asc = models.IntegerField(verbose_name="ソート順(昇順)", null=False, blank=False, default=0)
    is_primary = models.BooleanField(verbose_name="主要カードタイプ", default=False, help_text="デフォルト表示するカードタイプかどうか")

    def __str__(self):
        return self.name

    class Meta:
        verbose_name = "カードタイプ"
        verbose_name_plural = "カードタイプ"


class Klass(models.Model):  # "精像:天使/英知"
    cat1 = models.CharField(verbose_name="大分類", max_length=5, null=False, blank=False)
    cat2 = models.CharField(verbose_name="小分類1", max_length=5, null=True, blank=True)
    cat3 = models.CharField(verbose_name="小分類2", max_length=5, null=True, blank=True)
    sort_asc = models.IntegerField(verbose_name="ソート順(昇順)", null=False, blank=False, default=0)

    def __str__(self):
        if self.cat2:
            if self.cat3:
                return f"{self.cat1}:{self.cat2}/{self.cat3}"
            else:
                return f"{self.cat1}:{self.cat2}"
        else:
            return self.cat1

    class Meta:
        verbose_name = "種族"
        verbose_name_plural = "種族"


class Lrig(models.Model):
    name = models.CharField(verbose_name="名前", max_length=16, null=False, blank=False)
    code = models.CharField(verbose_name="内部値", max_length=16, null=False, blank=False)
    sort_asc = models.IntegerField(verbose_name="ソート順(昇順)", null=False, blank=False, default=0)

    def __str__(self):
        return f"{self.name} ({self.code})"

    class Meta:
        verbose_name = "ルリグタイプ"
        verbose_name_plural = "ルリグタイプ"


class Timing(models.Model):
    name = models.CharField(verbose_name="名前", max_length=16, null=False, blank=False)
    code = models.CharField(verbose_name="内部値", max_length=16, null=False, blank=False)
    bit = models.IntegerField(verbose_name="ビット値", null=False, blank=False, default=1 << 4)
    sort_asc = models.IntegerField(verbose_name="ソート順(昇順)", null=False, blank=False, default=0)

    def __str__(self):
        return f"{self.name} ({self.code})"

    class Meta:
        verbose_name = "使用タイミング"
        verbose_name_plural = "使用タイミング"


class Feature(models.Model):
    name = models.CharField(verbose_name="名前", max_length=16, null=False, blank=False)
    code = models.CharField(verbose_name="内部値", max_length=16, null=False, blank=False)
    sort_asc = models.IntegerField(verbose_name="ソート順(昇順)", null=False, blank=False, default=0)

    def __str__(self):
        return f"{self.name} ({self.code})"

    class Meta:
        verbose_name = "特性"
        verbose_name_plural = "特性"


class Card(models.Model):
    name = models.CharField(verbose_name="名前", max_length=256, null=False, blank=False, default='N/A')
    code = models.CharField(verbose_name="番号", max_length=16, null=False, blank=False, default='N/A', unique=True)
    pronunciation = models.CharField(verbose_name="読み方", max_length=128, null=False, blank=False, default='N/A')
    user = models.ManyToManyField(verbose_name="限定条件", to=Lrig, blank=True)
    color = models.IntegerField(verbose_name="色", null=False, blank=False, default=1 << 7)
    cost = models.CharField(verbose_name="使用コスト", max_length=16, null=True, blank=True)
    level = models.IntegerField(verbose_name="レベル", null=True, blank=True)
    limit = models.IntegerField(verbose_name="リミット", null=True, blank=True)
    limit_ex = models.IntegerField(verbose_name="リミット消費", null=True, blank=True)
    klass = models.ManyToManyField(verbose_name="種族", to=Klass, blank=True)
    product = models.IntegerField(verbose_name="収録商品", null=False, blank=False, default=0)
    card_type = models.IntegerField(verbose_name="カード種類", null=False, blank=False, default=0)
    power = models.CharField(verbose_name="パワー", max_length=5, null=True, blank=True)
    has_burst = models.IntegerField(verbose_name="バースト有無", null=False, blank=False, default=0)  # バーストあり1，なし2、無関係0
    skill_text = models.TextField(verbose_name="テキスト", null=True, blank=True)
    burst_text = models.TextField(verbose_name="テキスト(バースト)", null=True, blank=True)
    format = models.IntegerField(verbose_name="フォーマット", null=False, blank=False, default=0, choices=(
        (1, '(001) ディーヴァセレクション'),
        (3, '(011) キーセレクション'),
        (7, '(111) オールスター'),
    ))  # 001: diva, 010: key, 100: allstar
    story = models.CharField(verbose_name="ストーリー", max_length=16, null=True, blank=True)  # dissona
    rarity = models.CharField(verbose_name="レアリティ", max_length=8, null=True, blank=True, choices=(
        ('lr', 'LR'), ('lc', 'LC'), ('sr', 'SR'), ('r', 'R'), ('c', 'C'),
        ('stlr', 'STLR'), ('st', 'ST'), ('pr', 'PR'), ('sp', 'SP'), ('re', 'Re'), ('cb', 'CB'), ('pl', 'Pl'), ('l', 'L')
    ))
    timing = models.IntegerField(verbose_name="使用タイミング", null=True, blank=True, default=1 << 0)
    feature = models.ManyToManyField(verbose_name="特性", to=Feature, blank=True)
    url = models.URLField(verbose_name="詳細ページ",
                          null=True, blank=True
                          )

    feature_bits1 = models.BigIntegerField(verbose_name="効果1群", null=False, blank=False, default=0)
    feature_bits2 = models.BigIntegerField(verbose_name="効果2群", null=False, blank=False, default=0)
    burst_bits = models.BigIntegerField(verbose_name="ライフバースト効果", null=False, blank=False, default=0)
    ex1 = models.CharField(verbose_name="カード種特殊", max_length=256, null=True, blank=True)

    def __str__(self):
        return self.name

    class Meta:
        verbose_name = "カード"
        verbose_name_plural = "カード"


class RawCard(models.Model):
    """スクレイピングした生のカードデータを保存するモデル"""
    
    # 基本識別情報
    card_number = models.CharField(verbose_name="カード番号", max_length=20, unique=True, db_index=True)
    name = models.CharField(verbose_name="カード名", max_length=200)
    product = models.ForeignKey('Product', verbose_name="収録商品", on_delete=models.CASCADE, related_name='raw_cards', null=True, blank=True)
    
    # 生HTML（完全な状態で保持）
    raw_html = models.TextField(verbose_name="生HTML")
    
    # 事前解析済みフィールド
    skill_text = models.TextField(verbose_name="スキルテキスト", blank=True, help_text="cardSkillセクションのテキスト")
    life_burst_text = models.TextField(verbose_name="ライフバーストテキスト", blank=True, help_text="ライフバースト部分のテキスト")
    
    # メタデータ
    source_url = models.URLField(verbose_name="ソースURL", max_length=500)
    scraped_at = models.DateTimeField(verbose_name="スクレイピング日時", auto_now_add=True)
    last_analyzed_at = models.DateTimeField(verbose_name="最終解析日時", null=True, blank=True)
    
    # 解析ステータス
    is_analyzed = models.BooleanField(verbose_name="解析済み", default=False)
    analysis_error = models.TextField(verbose_name="解析エラー", blank=True)
    
    class Meta:
        verbose_name = "生カードデータ"
        verbose_name_plural = "生カードデータ"
        unique_together = ['card_number', 'product']
        ordering = ['product', 'card_number']
        indexes = [
            models.Index(fields=['card_number']),
            models.Index(fields=['scraped_at']),
            models.Index(fields=['is_analyzed']),
        ]
    
    def __str__(self):
        return f"{self.card_number} - {self.name}"


class RulePattern(models.Model):
    """rule_editorで作成されたパターンを保存するモデル"""
    
    keyword = models.CharField(verbose_name="キーワード", max_length=256)
    pattern = models.CharField(verbose_name="正規表現パターン", max_length=512)
    features = models.JSONField(verbose_name="検出フィーチャー", default=list)
    positive_examples = models.JSONField(verbose_name="マッチすべき例", default=list)
    negative_examples = models.JSONField(verbose_name="マッチすべきでない例", default=list)
    created_at = models.DateTimeField(verbose_name="作成日時", auto_now_add=True)
    updated_at = models.DateTimeField(verbose_name="更新日時", auto_now=True)
    is_active = models.BooleanField(verbose_name="有効", default=True)
    
    class Meta:
        verbose_name = "ルールパターン"
        verbose_name_plural = "ルールパターン"
        db_table = 'wix_rule_pattern'
        ordering = ['-created_at']
    
    def __str__(self):
        return f"{self.keyword} - {self.pattern}"


class CardFeatureOverride(models.Model):
    """カードフィーチャーの手動修正データを保存するモデル"""
    
    pronunciation = models.CharField(
        verbose_name="読み方", 
        max_length=200, 
        unique=True, 
        db_index=True,
        help_text="同じ読み方のカードは全て同じフィーチャーが適用されます"
    )
    fixed_bits1 = models.BigIntegerField(
        verbose_name="修正済み効果1群",
        help_text="手動で修正されたCardFeatureのビット値（1群）"
    )
    fixed_bits2 = models.BigIntegerField(
        verbose_name="修正済み効果2群",
        help_text="手動で修正されたCardFeatureのビット値（2群）"
    )
    fixed_burst_bits = models.BigIntegerField(
        verbose_name="修正済みライフバースト効果",
        help_text="手動で修正されたBurstFeatureのビット値"
    )
    created_at = models.DateTimeField(verbose_name="作成日時", auto_now_add=True)
    updated_at = models.DateTimeField(verbose_name="更新日時", auto_now=True)
    note = models.TextField(
        verbose_name="メモ", 
        blank=True,
        help_text="修正理由や特記事項"
    )
    
    class Meta:
        verbose_name = "カードフィーチャー修正"
        verbose_name_plural = "カードフィーチャー修正"
        db_table = 'wix_card_feature_override'
        ordering = ['pronunciation']
    
    def __str__(self):
        return f"{self.pronunciation} - 修正済み"
