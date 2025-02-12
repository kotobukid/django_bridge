from django.core.management.base import BaseCommand
from wix.models import Card


class Command(BaseCommand):
    help = ""

    def handle(self, *args, **options):
        cards_all = Card.objects.all()
        for card in cards_all:
            print(card.name)
            card.bool1 = False
            card.save()
