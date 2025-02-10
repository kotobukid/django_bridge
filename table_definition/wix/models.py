from django.db import models


class Card(models.Model):
    name = models.CharField(max_length=256, null=False, blank=False, default='taro')
    created_at = models.DateTimeField(auto_now_add=True, null=False, blank=False)
    bool1 = models.BooleanField(default=False)
    option1 = models.CharField(max_length=128, null=True, blank=True)