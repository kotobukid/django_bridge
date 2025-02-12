from django.db import models


class Card(models.Model):
    name = models.CharField(max_length=256, null=False, blank=False, default='taro')
    created_at = models.DateTimeField(auto_now_add=True, null=False, blank=False)
    bool1 = models.BooleanField(default=False)
    option1 = models.CharField(max_length=128, null=True, blank=True)
    info = models.JSONField(null=True, blank=True)

    def __str__(self):
        return self.name


class Tag(models.Model):
    label = models.CharField(max_length=128, null=False, blank=False)

    def __str__(self):
        return self.label
