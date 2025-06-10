#!/usr/bin/env python3
"""
HTML構造分析スクリプト - 6つの追加フィールド用
パワー、レベル、リミット、リミット消費、ストーリー、使用タイミングの位置を調査

Usage: python scripts/analyze_html_structure.py
"""
import os
import sys

# プロジェクトルートからの相対パスでDjango設定
project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
table_definition_path = os.path.join(project_root, 'table_definition')
sys.path.append(table_definition_path)

os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'table_definition.settings')

import django
django.setup()

from wix.models import RawCard
import random

def extract_dd_elements(html):
    """HTMLからdd要素を順番に抽出"""
    dd_elements = []
    
    # <dt>カード種類</dt> または <dt>種類</dt> 以降の<dd>要素を抽出
    start_patterns = ["<dt>カード種類</dt>", "<dt>種類</dt>"]
    start_index = None
    
    for pattern in start_patterns:
        if pattern in html:
            start_index = html.find(pattern)
            break
    
    if start_index is None:
        return dd_elements
    
    current_pos = start_index
    
    # <dd>要素を順番に収集
    while True:
        dd_start = html.find("<dd>", current_pos)
        if dd_start == -1:
            break
        
        dd_end = html.find("</dd>", dd_start)
        if dd_end == -1:
            break
        
        dd_content = html[dd_start + 4:dd_end]
        dd_elements.append(dd_content.strip())
        current_pos = dd_end + 5
    
    return dd_elements

def analyze_card_structure(raw_card):
    """個別カードのHTML構造を分析"""
    print(f"\nCard: {raw_card.card_number} - {raw_card.name}")
    print("=" * 60)
    
    dd_elements = extract_dd_elements(raw_card.raw_html)
    
    if not dd_elements:
        print("ERROR: dd要素が抽出できませんでした")
        return
    
    print(f"Total dd elements: {len(dd_elements)}")
    print("-" * 40)
    
    for i, content in enumerate(dd_elements):
        # 長いテキストは省略
        display_content = content[:50] + "..." if len(content) > 50 else content
        print(f"dd[{i:2d}]: {display_content}")
    
    # カードタイプの確認
    if dd_elements:
        card_type = dd_elements[0]
        is_lrig = "ルリグ" in card_type
        print(f"\nCard Type: {card_type} (Lrig: {is_lrig})")
        
        # 既知のフィールド位置
        print("\n既知のフィールド位置:")
        if is_lrig:
            print(f"  グロウコスト (dd[4]): {dd_elements[4] if len(dd_elements) > 4 else 'N/A'}")
        else:
            print(f"  コスト (dd[5]): {dd_elements[5] if len(dd_elements) > 5 else 'N/A'}")

def main():
    """メイン分析処理"""
    print("HTML構造分析 - 6つの追加フィールド調査")
    print("=" * 60)
    print("対象フィールド: パワー、レベル、リミット、リミット消費、ストーリー、使用タイミング")
    
    # 各カードタイプから数枚ずつサンプリング
    card_types = ["ルリグ", "シグニ", "スペル", "アーツ", "ピース", "クラフト"]
    
    for card_type in card_types:
        print(f"\n\n{card_type}カードのサンプル:")
        print("=" * 60)
        
        # 該当するカードを検索（RawCardのHTMLから判定）
        matching_cards = []
        all_cards = list(RawCard.objects.all())
        
        for card in all_cards:
            dd_elements = extract_dd_elements(card.raw_html)
            if dd_elements and card_type in dd_elements[0]:
                matching_cards.append(card)
                if len(matching_cards) >= 3:  # 各タイプ3枚まで
                    break
        
        if not matching_cards:
            print(f"  {card_type}カードが見つかりませんでした")
            continue
        
        for card in matching_cards:
            analyze_card_structure(card)

if __name__ == "__main__":
    main()