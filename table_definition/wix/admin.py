from django.contrib import admin

from .models import Card, CardType, Color, Lrig, Product, Klass, Feature, Timing

admin.site.register(Card)
admin.site.register(CardType)
admin.site.register(Color)
admin.site.register(Lrig)
admin.site.register(Product)
admin.site.register(Klass)
admin.site.register(Feature)
admin.site.register(Timing)