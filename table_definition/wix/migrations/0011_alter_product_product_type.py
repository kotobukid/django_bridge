# Generated by Django 5.2.1 on 2025-06-02 16:02

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('wix', '0010_card_ex1'),
    ]

    operations = [
        migrations.AlterField(
            model_name='product',
            name='product_type',
            field=models.CharField(choices=[('bo', 'ブースター'), ('st', 'スターター'), ('pr', 'プロモーション'), ('sp', 'スペシャル')], max_length=2, verbose_name='商品タイプ'),
        ),
    ]
