# Generated migration for rule_pattern table

from django.db import migrations, models
import django.contrib.postgres.fields


class Migration(migrations.Migration):

    dependencies = [
        ('wix', '0018_alter_card_pronunciation'),
    ]

    operations = [
        migrations.CreateModel(
            name='RulePattern',
            fields=[
                ('id', models.AutoField(auto_created=True, primary_key=True, serialize=False, verbose_name='ID')),
                ('keyword', models.CharField(max_length=256, verbose_name='キーワード')),
                ('pattern', models.CharField(max_length=512, verbose_name='正規表現パターン')),
                ('features', models.JSONField(verbose_name='検出フィーチャー', default=list)),
                ('positive_examples', models.JSONField(verbose_name='マッチすべき例', default=list)),
                ('negative_examples', models.JSONField(verbose_name='マッチすべきでない例', default=list)),
                ('created_at', models.DateTimeField(auto_now_add=True, verbose_name='作成日時')),
                ('updated_at', models.DateTimeField(auto_now=True, verbose_name='更新日時')),
                ('is_active', models.BooleanField(default=True, verbose_name='有効')),
            ],
            options={
                'verbose_name': 'ルールパターン',
                'verbose_name_plural': 'ルールパターン',
                'db_table': 'wix_rule_pattern',
                'ordering': ['-created_at'],
            },
        ),
    ]