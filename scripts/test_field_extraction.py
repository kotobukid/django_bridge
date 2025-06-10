#!/usr/bin/env python3
"""
フィールド抽出テストスクリプト - 各カードタイプでの適切なフィールド抽出をテスト

Usage: python scripts/test_field_extraction.py
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

from wix.models import RawCard, Card

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

def test_card_type_extraction(card_type_name, expected_fields):
    """特定のカードタイプの抽出テスト"""
    print(f"\n{'='*60}")
    print(f"Testing {card_type_name} Cards")
    print(f"{'='*60}")
    
    # RawCardから該当するカードを検索
    matching_raw_cards = []
    for raw_card in RawCard.objects.all():
        dd_elements = extract_dd_elements(raw_card.raw_html)
        if dd_elements and card_type_name in dd_elements[0]:
            matching_raw_cards.append(raw_card)
            if len(matching_raw_cards) >= 3:  # 3枚まで
                break
    
    if not matching_raw_cards:
        print(f"❌ No {card_type_name} cards found in RawCard")
        return
    
    print(f"Found {len(matching_raw_cards)} {card_type_name} cards for testing")
    
    for i, raw_card in enumerate(matching_raw_cards):
        print(f"\n--- Test Card {i+1}: {raw_card.card_number} - {raw_card.name[:30]}... ---")
        
        dd_elements = extract_dd_elements(raw_card.raw_html)
        if not dd_elements:
            print("❌ Failed to extract dd elements")
            continue
        
        print("Raw HTML dd elements:")
        for idx, content in enumerate(dd_elements[:12]):  # 最初の12要素のみ表示
            display_content = content[:30] + "..." if len(content) > 30 else content
            print(f"  dd[{idx:2d}]: {display_content}")
        
        # 対応するCardレコードを検索
        try:
            card = Card.objects.get(code=raw_card.card_number)
            print(f"\n✅ Found corresponding Card record")
            
            # 期待されるフィールドをテスト
            print("\nField Extraction Results:")
            for field_name, dd_index, expected_type in expected_fields:
                raw_value = dd_elements[dd_index] if len(dd_elements) > dd_index else "N/A"
                card_value = getattr(card, field_name, "FIELD_NOT_FOUND")
                
                # 値の評価
                if raw_value == "-" or raw_value == "":
                    expected_card_value = None
                else:
                    if expected_type == "int":
                        try:
                            expected_card_value = int(raw_value)
                        except ValueError:
                            expected_card_value = None
                    elif expected_type == "string":
                        expected_card_value = raw_value
                    elif expected_type == "timing":
                        # タイミングマッピング
                        timing_map = {"メインフェイズ": 1, "アタックフェイズ": 2}
                        expected_card_value = timing_map.get(raw_value)
                    else:
                        expected_card_value = raw_value
                
                # 結果表示
                status = "✅" if card_value == expected_card_value else "❌"
                print(f"  {status} {field_name:12s}: raw='{raw_value}' -> card={card_value} (expected={expected_card_value})")
                
        except Card.DoesNotExist:
            print(f"❌ No corresponding Card record found for {raw_card.card_number}")

def main():
    """メインテスト実行"""
    print("Field Extraction Test for All Card Types")
    print("=" * 60)
    
    # 各カードタイプと期待されるフィールドの定義
    test_cases = [
        ("ルリグ", [
            ("level", 3, "int"),         # レベル
            ("cost", 4, "string"),       # グロウコスト（文字列として保存）
            ("limit", 6, "int"),         # リミット
            ("power", 7, "string"),      # パワー（ルリグは通常None）
            ("timing", 9, "timing"),     # タイミング（ルリグは通常None）
        ]),
        ("シグニ", [
            ("level", 3, "int"),         # レベル
            ("cost", 5, "string"),       # コスト
            ("limit", 6, "int"),         # リミット（シグニは通常None）
            ("power", 7, "string"),      # パワー
            ("timing", 9, "timing"),     # タイミング（シグニは通常None）
        ]),
        ("スペル", [
            ("level", 3, "int"),         # レベル（スペルは通常None）
            ("cost", 5, "string"),       # コスト
            ("limit", 6, "int"),         # リミット（スペルは通常None）
            ("power", 7, "string"),      # パワー（スペルは通常None）
            ("timing", 9, "timing"),     # タイミング（スペルは通常None）
        ]),
        ("アーツ", [
            ("level", 3, "int"),         # レベル（アーツは通常None）
            ("cost", 5, "string"),       # コスト
            ("limit", 6, "int"),         # リミット（アーツは通常None）
            ("power", 7, "string"),      # パワー（アーツは通常None）
            ("timing", 9, "timing"),     # タイミング
        ]),
        ("ピース", [
            ("level", 3, "int"),         # レベル（ピースは通常None）
            ("cost", 5, "string"),       # コスト
            ("limit", 6, "int"),         # リミット（ピースは通常None）
            ("power", 7, "string"),      # パワー（ピースは通常None）
            ("timing", 9, "timing"),     # タイミング
        ]),
    ]
    
    # 各カードタイプをテスト
    for card_type, expected_fields in test_cases:
        test_card_type_extraction(card_type, expected_fields)
    
    print(f"\n{'='*60}")
    print("Test Summary")
    print("=" * 60)
    print("✅ = Field extracted correctly")
    print("❌ = Field extraction failed or mismatch")
    print("\nNote: Some fields may legitimately be None/empty for certain card types")

if __name__ == "__main__":
    main()