# Generated by Django 5.1.6 on 2025-02-13 09:52

from django.db import migrations


class Migration(migrations.Migration):

    dependencies = [
        ('wix', '0006_rename_tags_card_tagsa'),
    ]

    operations = [
        migrations.RenameField(
            model_name='card',
            old_name='tagsa',
            new_name='tags',
        ),
    ]
