# Generated by Django 5.1.6 on 2025-03-21 07:43

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('wix', '0009_alter_card_feature_bits1_alter_card_feature_bits2'),
    ]

    operations = [
        migrations.AddField(
            model_name='card',
            name='ex1',
            field=models.CharField(blank=True, max_length=256, null=True, verbose_name='カード種特殊'),
        ),
    ]
