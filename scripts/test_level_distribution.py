#!/usr/bin/env python

"""Test script to check the distribution of level values in the database."""

import os
import sys
from collections import Counter

# Add the project directory to Python path
project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(project_root, 'table_definition'))

# Setup Django
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')
import django
django.setup()

from wix.models import Card

def check_level_distribution():
    """Check the distribution of level values in the Card table."""
    # Get all cards with level information
    cards = Card.objects.exclude(level__isnull=True)
    
    print(f"Total cards with level information: {cards.count()}")
    
    # Count level distribution
    level_counter = Counter()
    for card in cards:
        level_counter[card.level] += 1
    
    print("\nLevel distribution:")
    for level, count in sorted(level_counter.items()):
        print(f"  Level {level}: {count} cards")
    
    # Get some example cards for each level
    print("\nExample cards for each level:")
    for level in sorted(level_counter.keys())[:6]:  # Show first 6 levels
        examples = Card.objects.filter(level=level)[:3]
        print(f"\n  Level {level}:")
        for card in examples:
            print(f"    - {card.name} ({card.code})")

if __name__ == "__main__":
    check_level_distribution()