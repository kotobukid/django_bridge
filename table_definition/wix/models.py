from django.db import models


class Product(models.Model):
    name = models.CharField(max_length=256, null=False, blank=False)
    product_code = models.CharField(max_length=128, null=False, blank=False)
    url = models.URLField(null=True, blank=True)
    product_type = models.CharField(
        max_length=2, null=False, blank=False,
        choices=[("bo", "ブースター"), ("st", "スターター"), ("pr", "プロモーション")]
    )
    sort_asc = models.IntegerField(null=False, blank=False, default=0)

    def __str__(self):
        return self.name

    class Meta:
        verbose_name = "製品"
        verbose_name_plural = "製品"


class Color(models.Model):  # 白、赤…無色
    name = models.CharField(max_length=256, null=False, blank=False)
    code = models.CharField(max_length=16, null=False, blank=False)
    bit = models.IntegerField(null=False, blank=False, default=1 << 7)
    sort_asc = models.IntegerField(null=False, blank=False, default=0)

    def __str__(self):
        return f"{self.name} ({self.code.upper()})"

    class Meta:
        verbose_name = "色"
        verbose_name_plural = "色"


class CardType(models.Model):  # シグニ、ルリグ…
    name = models.CharField(max_length=8, null=False, blank=False)
    code = models.CharField(max_length=32, null=False, blank=False)
    sort_asc = models.IntegerField(null=False, blank=False, default=0)

    def __str__(self):
        return self.name

    class Meta:
        verbose_name = "カードタイプ"
        verbose_name_plural = "カードタイプ"


class Klass(models.Model):  # "精像:天使/英知"
    cat1 = models.CharField(max_length=5, null=False, blank=False)
    cat2 = models.CharField(max_length=5, null=True, blank=True)
    cat3 = models.CharField(max_length=5, null=True, blank=True)
    sort_asc = models.IntegerField(null=False, blank=False, default=0)

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
    name = models.CharField(max_length=16, null=False, blank=False)
    code = models.CharField(max_length=16, null=False, blank=False)
    sort_asc = models.IntegerField(null=False, blank=False, default=0)

    def __str__(self):
        return f"{self.name} ({self.code})"

    class Meta:
        verbose_name = "ルリグタイプ"
        verbose_name_plural = "ルリグタイプ"


class Timing(models.Model):
    name = models.CharField(max_length=16, null=False, blank=False)
    code = models.CharField(max_length=16, null=False, blank=False)
    sort_asc = models.IntegerField(null=False, blank=False, default=0)

    def __str__(self):
        return f"{self.name} ({self.code})"

    class Meta:
        verbose_name = "使用タイミング"
        verbose_name_plural = "使用タイミング"


class Feature(models.Model):
    name = models.CharField(max_length=16, null=False, blank=False)
    code = models.CharField(max_length=16, null=False, blank=False)
    sort_asc = models.IntegerField(null=False, blank=False, default=0)

    def __str__(self):
        return f"{self.name} ({self.code})"

    class Meta:
        verbose_name = "効果"
        verbose_name_plural = "効果"


class Card(models.Model):
    name = models.CharField(max_length=256, null=False, blank=False, default='N/A')
    code = models.CharField(max_length=16, null=False, blank=False, default='N/A')
    pronunciation = models.CharField(max_length=32, null=False, blank=False, default='N/A')
    user = models.ManyToManyField(to=Lrig, blank=True)
    color = models.IntegerField(null=False, blank=False, default=1 << 7)
    cost = models.CharField(max_length=16, null=True, blank=True)
    level = models.IntegerField(null=True, blank=True)
    limit = models.IntegerField(null=True, blank=True)
    limit_ex = models.IntegerField(null=True, blank=True)
    klass = models.ManyToManyField(to=Klass, blank=True)
    product = models.ForeignKey(
        Product, on_delete=models.CASCADE, null=True, blank=True
    )
    card_type = models.ForeignKey(CardType, on_delete=models.CASCADE, null=True, blank=True)
    power = models.CharField(max_length=5, null=True, blank=True)
    has_burst = models.BooleanField(default=False)
    skill_text = models.TextField(null=True, blank=True)
    burst_text = models.TextField(null=True, blank=True)
    format = models.IntegerField(null=False, blank=False, default=0, choices=(
        (1, '(001) ディーヴァセレクション'),
        (3, '(011) キーセレクション'),
        (7, '(111) オールスター'),
    ))  # 001: diva, 010: key, 100: allstar
    story = models.CharField(max_length=16, null=True, blank=True)  # dissona
    rarity = models.CharField(max_length=8, null=True, blank=True, choices=(
        ('lr', 'LR'), ('lc', 'LC'), ('sr', 'SR'), ('r', 'R'), ('c', 'C'),
        ('stlr', 'STLR'), ('st', 'ST'), ('pr', 'PR'), ('sp', 'SP'), ('re', 'Re'), ('cb', 'CB'), ('pl', 'Pl'), ('l', 'L')
    ))
    timing = models.ManyToManyField(to=Timing, blank=True)
    feature = models.ManyToManyField(to=Feature, blank=True)
    url = models.URLField(
        null=True, blank=True
    )

    def __str__(self):
        return self.name

    class Meta:
        verbose_name = "カード"
        verbose_name_plural = "カード"
