# Generated by Django 5.1.6 on 2025-02-12 07:34

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('wix', '0002_card_bool1_card_option1_alter_card_name'),
    ]

    operations = [
        migrations.CreateModel(
            name='Tag',
            fields=[
                ('id', models.BigAutoField(auto_created=True, primary_key=True, serialize=False, verbose_name='ID')),
                ('label', models.CharField(max_length=128)),
            ],
        ),
        migrations.AddField(
            model_name='card',
            name='info',
            field=models.JSONField(blank=True, null=True),
        ),
    ]
