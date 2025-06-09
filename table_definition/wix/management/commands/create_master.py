from linecache import updatecache

from django.core.management.base import BaseCommand
from wix.models import Color, CardType, Lrig, Klass, Timing, Product


class Command(BaseCommand):
    help = ""

    def handle(self, *args, **options):
        update_color()
        update_card_type()
        update_lrig()
        update_klass()
        update_timing()
        update_product()


def update_color():
    color_source = (
        ('w', '白', 1 << 1, 1),
        ('u', '青', 1 << 2, 3),
        ('r', '赤', 1 << 3, 2),
        ('k', '黒', 1 << 4, 4),
        ('g', '緑', 1 << 5, 5),
        ('l', '無', 1 << 6, 6),
        ('x', '?', 1 << 7, 7),
    )
    colors_existing = Color.objects.all()
    for color in color_source:
        print(color)
        c_ex = colors_existing.filter(code=color[0])

        if c_ex.count() == 0:
            new_color = Color(code=color[0], name=color[1], bit=color[2], sort_asc=color[3])
            new_color.save()
        else:
            if c_ex[0].name != color[1]:
                target_color = Color.objects.get(code=color[0])
                target_color[0].name = color[1]
                target_color[0].bit = color[2]
                target_color[0].sort_asc = color[3]
                target_color[0].save()
    print('color update complete.')


def update_card_type():
    type_source = (
        ('lrig', 'ルリグ', 0),
        ('arts', 'アーツ', 1),
        ('lrig_assist', 'アシストルリグ', 2),
        ('piece', 'ピース', 3),
        ('signi', 'シグニ', 4),
        ('spell', 'スペル', 5),
        ('resona', 'レゾナ', 6),
        ('key', 'キー', 7),
        ('arts_craft', 'クラフトアーツ', 8),
        ('signi_craft', 'クラフトシグニ', 9),
        ('spell_craft', 'クラフトスペル', 10),
        ('piece_relay', 'リレーピース', 11),
        ('piece_craft', 'クラフトピース', 12),
        ('resona_craft', 'クラフトレゾナ', 13),
        ('token', 'トークン', 100),
        ('coin', 'コイン', 101),
    )
    types_existing = CardType.objects.all()
    for _type in type_source:
        print(_type)
        ctypes = types_existing.filter(code=_type[0])

        if ctypes.count() == 0:
            new_type = CardType(code=_type[0], name=_type[1], sort_asc=_type[2])
            new_type.save()
        else:
            if ctypes[0].name != _type[1]:
                target_type = CardType.objects.get(code=_type[0])
                target_type.name = _type[1]
                target_type.sort_asc = _type[2]
                target_type.save()
    print('card type update complete.')


def update_lrig():
    lrig_source = (
        ('tama', 'タマ', 0),
        ('tawil', 'タウィル', 1),
        ('remember', 'リメンバ', 2),
        ('sashe', 'サシェ', 3),
        ('dona', 'ドーナ', 4),
        ('emma', 'エマ', 5),
        ('lyze', 'リゼ', 6),
        ('ange', 'アンジュ', 7),
        ('akino', 'アキノ', 8),
        ('lion', 'LION', 9),
        ('nova', 'ノヴァ', 10),
        ('yukayuka', 'ゆかゆか', 11),
        ('azusa', 'アズサ', 12),
        ('saori', 'サオリ', 13),
        ('hoshu', '補習授業部', 1013),
        ('gabriela', 'ガブリエラ', 14),
        ('ruuko', 'るう子', 15),
        ('hanayo', '花代', 16),
        ('yuzuki', 'ユヅキ', 17),
        ('ril', 'リル', 18),
        ('carnival', 'カーニバル', 19),
        ('layla', 'レイラ', 20),
        ('lov', 'LoV', 21),
        ('hirana', 'ヒラナ', 22),
        ('lovit', 'LOVIT', 23),
        ('ex', 'エクス', 24),
        ('azaela', 'アザエラ', 25),
        ('chiyori', 'ちより', 26),
        ('piruluk', 'ピルルク', 27),
        ('eldora', 'エルドラ', 28),
        ('milulun', 'ミルルン', 29),
        ('aya', 'あや', 30),
        ('rei', 'レイ', 31),
        ('tamago', 'タマゴ', 32),
        ('madoka', 'マドカ', 33),
        ('mikomiko', 'みこみこ', 34),
        ('neru', 'ネル', 35),
        ('miyako', 'ミヤコ', 36),
        ('cac', 'C&C', 1036),
        ('michaela', 'ミカエラ', 37),
        ('akira', 'あきら', 38),
        ('midoriko', '緑子', 39),
        ('ann', 'アン', 40),
        ('aiyai', 'アイヤイ', 41),
        ('mel', 'メル', 42),
        ('mama', 'ママ', 43),
        ('at', 'アト', 44),
        ('wolf', 'WOLF', 45),
        ('bang', 'バン', 46),
        ('sanga', 'サンガ', 47),
        ('shiroko', 'シロコ', 48),
        ('yukari', 'ユカリ', 49),
        ('hoshino', 'ホシノ', 50),
        ('taisaku', '対策委員会', 1049),
        ('hitoe', 'ひとえ', 50),
        ('urith', 'ウリス', 51),
        ('iona', 'イオナ', 52),
        ('umr', 'ウムル', 53),
        ('myu', 'ミュウ', 54),
        ('alfou', 'アルフォウ', 55),
        ('hanare', 'ハナレ', 56),
        ('nanashi', 'ナナシ', 57),
        ('guzuko', 'グズ子', 58),
        ('toko', 'とこ', 59),
        ('muzica', 'ムジカ', 60),
        ('deus', 'デウス', 61),
        ('machina', 'マキナ', 62),
        ('mahomaho', 'まほまほ', 63),
        ('mito', '美兎', 64),
        ('hina', 'ヒナ', 65),
        ('shun', 'シュン', 66),
        ('fuuki', '風紀委員会', 1066),
        ('mugen', '夢限', 67),
        ('hatena', '？', 68),
        ('nijisanji', 'にじさんじ', 69),
    )
    lrigs = Lrig.objects.all()
    for lrig in lrig_source:
        print(lrig)
        l = lrigs.filter(code=lrig[0])

        if l.count() == 0:
            new_lrig = Lrig(code=lrig[0], name=lrig[1], sort_asc=lrig[2])
            new_lrig.save()
        else:
            if l[0].name != lrig[1]:
                target_lrig = Lrig.objects.get(code=lrig[0])
                target_lrig.name = lrig[1]
                target_lrig.sort_asc = lrig[2]
                target_lrig.save()
    print('lrig update complete.')


def update_klass():
    klass_source = (
        (1000, '精像', '天使'),
        (1001, '精像', '悪魔'),
        # ('精像', '天使', '悪魔'),
        (1002, '精像', '美巧'),
        (1003, '精像', '英知'),
        # ('精像', '精像：天使/英知'),
        (1004, '精像', '武勇'),
        # ('精像', '精像：武勇/美巧'),
        (1005, '精武', 'アーム'),
        (1006, '精武', 'ウェポン'),
        # ('精武', '精武：アーム/ウェポン'),
        (1007, '精武', '遊具'),
        (1008, '精武', '毒牙'),
        (1009, '精武', 'トリック'),
        # ('精武', '精武：ウェポン/トリック'),
        (1010, '精羅', '鉱石'),
        (1011, '精羅', '宝石'),
        (1012, '精羅', '植物'),
        (1013, '精羅', '原子'),
        (1014, '精羅', '宇宙'),
        (1015, '精羅', '微菌'),
        (1016, '精械', '電機'),
        (1017, '精械', '古代兵器'),
        (1018, '精械', '迷宮'),
        (1019, '精械', '調理'),
        # ('精械', '迷宮/調理'),
        (1020, '精械', '乗機'),
        # ('精械', '精械：乗機/古代兵器'),
        (1021, '精械', '紅蓮'),
        # ('精械', '紅蓮/古代兵器'),
        (1022, '精生', '水獣'),
        (1023, '精生', '空獣'),
        (1024, '精生', '地獣'),
        # ('精生', '精生：空獣/地獣'),
        (1025, '精生', '龍獣'),
        (1026, '精生', '凶蟲'),
        (1027, '精生', '怪異'),
        (1028, '精元',),
        (0, '奏像', '天使'),
        (1, '奏像', '悪魔'),
        (2, '奏像', '美巧'),
        (3, '奏像', '英知'),
        (4, '奏像', '武勇'),
        (5, '奏像', 'プリパラ'),
        (6, '奏武', 'アーム'),
        (7, '奏武', 'ウェポン'),
        (8, '奏武', '遊具'),
        (9, '奏武', '毒牙'),
        (10, '奏武', 'トリック'),
        (11, '奏武', 'ブルアカ'),
        (12, '奏羅', '宝石'),
        (13, '奏羅', '植物'),
        (14, '奏羅', '原子'),
        (15, '奏羅', '宇宙'),
        (16, '奏羅', '微菌'),
        (17, '奏械', '電機'),
        (18, '奏械', '古代兵器'),
        (19, '奏械', '迷宮'),
        (20, '奏械', '調理'),
        (21, '奏械', '乗機'),
        (22, '奏械', 'バーチャル'),
        (23, '奏械', '電音部'),
        (24, '奏生', '水獣'),
        (25, '奏生', '空獣'),
        (26, '奏生', '地獣'),
        (27, '奏生', '龍獣'),
        (28, '奏生', '凶蟲'),
        (29, '奏生', '怪異'),
        (30, '奏元',),
        (31, '解放派',),
        (32, '闘争派',),
        (33, '防衛派',),
    )
    klasses = Klass.objects.all()

    def find_klass(klasses_all, klass):
        length = len(klass)
        l = Klass.objects.none()
        sort_asc = klass[0]
        cat1 = klass[1]
        cat2 = klass[2] if length > 2 else ''
        cat3 = klass[3] if length > 3 else ''

        if length == 2:
            l = klasses_all.filter(cat1=cat1)
        elif length == 3:
            l = klasses_all.filter(cat1=cat1, cat2=cat2)
        elif length == 4:
            l = klasses_all.filter(cat1=cat1, cat2=cat2, cat3=cat3)
        return l

    for klass in klass_source:
        print(klass)

        l = find_klass(klasses, klass)

        length = len(klass)
        sort_asc = klass[0]
        cat1 = klass[1]
        cat2 = klass[2] if length > 2 else ''
        cat3 = klass[3] if length > 3 else ''

        if l.count() == 0:
            new_klass = Klass(cat1=cat1, cat2=cat2, cat3=cat3, sort_asc=sort_asc)
            print(new_klass)
            new_klass.save()
        else:
            if l[0].cat1 != cat1 or l[0].cat2 != cat2 or l[0].cat3 != cat3 or l[0].sort_asc != sort_asc:
                target_klass = find_klass(klasses, klass)[0]

                target_klass.sort_asc = sort_asc
                target_klass.cat1 = cat1
                target_klass.cat2 = cat2
                target_klass.cat3 = cat3
                target_klass.save()
    print('klass update complete.')


def update_timing():
    timing_source = (
        ('main', 'メインフェイズ', 1 << 1, 0),
        ('attack', 'アタックフェイズ', 1 << 2, 1),
        ('spellcutin', 'スペルカットイン', 1 << 3, 2),
    )
    timing_existing = Timing.objects.all()
    for timing in timing_source:
        print(timing)
        t_ex = timing_existing.filter(code=timing[0])

        if t_ex.count() == 0:
            new_timing = Timing(code=timing[0], name=timing[1], bit=timing[2], sort_asc=timing[3])
            new_timing.save()
        else:
            if t_ex[0].name != timing[1] or t_ex[0].sort_asc != timing[2]:
                target_timing = Timing.objects.get(code=timing[0])
                target_timing.name = timing[1]
                target_timing.bit = timing[2]
                target_timing.sort_asc = timing[3]
                target_timing.save()
    print('timing update complete.')


def update_product():
    products_source = (
        (129, "st", "WX25-CD1", "ブルーアーカイブ〔WX25-CD1〕"),
        (130, "st", "WX24-D5", "BLACK ALT DESIRE〔WX24-D5〕"),
        (131, "st", "WX24-D4", "GREEN ALT WANNA〔WX24-D4〕"),
        (132, "st", "WX24-D3", "BLUE ALT APPLI〔WX24-D3〕"),
        (133, "st", "WX24-D2", "RED ALT AMBITION〔WX24-D2〕"),
        (134, "st", "WX24-D1", "WHITE ALT HOPE〔WX24-D1〕"),
        (135, "st", "WXDi-D09", "SUPER DIVA DECK DOUBLE HEROINES -ピルルク＆ヒラナ-〔WXDi-D09〕"),
        (136, "st", "WXDi-D08", "DIVA DEBUT DECK WHITE HOPE〔WXDi-D08〕"),
        (137, "st", "WXDi-D07", "TOP DIVA DECK D・X・M〔WXDi-D07〕"),
        (138, "st", "WXDi-D06", "DIVA DEBUT DECK DIAGRAM〔WXDi-D06〕"),
        (139, "st", "WXDi-D05", "DIVA DEBUT DECK うちゅうのはじまり〔WXDi-D05〕"),
        (140, "st", "WXDi-D04", "DIVA DEBUT DECK Card Jockey〔WXDi-D04〕"),
        (141, "st", "WXDi-D03", "DIVA DEBUT DECK No Limit〔WXDi-D03〕"),
        (142, "st", "WXDi-D02", "DIVA DEBUT DECK にじさんじ ver.さんばか〔WXDi-D02〕"),
        (143, "st", "WXDi-D01", "ANCIENT SURPRISE〔WXDi-D01〕"),
        (310, "st", "WDA-F05", "グズ子ではじめるウィクロスASは強烈連携攻撃で勝つ!〔WXA-DF05〕"),
        (311, "st", "WDA-F04", "ドーナではじめるウィクロスASは強烈全体強化で勝つ!〔WXA-DF04〕"),
        (312, "st", "WDA-F03", "遊月ではじめるウィクロスASは強烈焼却で勝つ!〔WXA-DF03〕"),
        (313, "st", "WDA-F02", "ピルルクではじめるウィクロスASは強烈手札破壊で勝つ!〔WXA-DF02〕"),
        (314, "st", "WDA-F01", "タマではじめるウィクロスASは強烈連続攻撃で勝つ!〔WXA-DF01〕"),
        (315, "st", "WDK-17", "ブラックアルフォウ〔WDK-17〕"),
        (316, "st", "WDK-16", "にじさんじウィクロスバトルセット〔WDK-16〕"),
        (317, "st", "WDK-15", "ブラックナナシ〔WDK-15〕"),
        (318, "st", "WDK-14", "レッドタウィル〔WDK-14〕"),
        (319, "st", "WDK-13", "ブラックミュウ〔WDK-13〕"),
        (320, "st", "WDK-12", "グリーンメル〔WDK-12〕"),
        (321, "st", "WDK-F05", "カーニバルではじめるウィクロスは墓地活用で勝つ!〔WXK-DF05〕"),
        (322, "st", "WDK-F04", "リルではじめるウィクロスは進化して勝つ!〔WXK-DF04〕"),
        (323, "st", "WDK-F03", "タマではじめるウィクロスは手札に戻して勝つ!〔WXK-DF03〕"),
        (324, "st", "WDK-F02", "グズ子ではじめるウィクロスは山札操作で勝つ！〔WXK-DF02〕"),
        (325, "st", "WDK-F01", "ピルルクではじめるウィクロスは手札破壊で勝つ!〔WXK-DF01〕"),
        (326, "st", "WDK-11", "ホワイトエマ〔WDK-11〕"),
        (327, "st", "WDK-10", "ブラックウリス〔WDK-10〕"),
        (328, "st", "WDK-09", "ブルーウムル〔WDK-09〕"),
        (329, "st", "WDK-08", "デュアルブラスト〔WDK-08〕"),
        (330, "st", "WDK-07", "デュアルプラント〔WDK-07〕"),
        (331, "st", "WDK-06", "デュアルブラッド〔WDK-06〕"),
        (332, "st", "WDK-05", "デュアルペイルネス〔WDK-05〕"),
        (333, "st", "WDK-04", "ブラックダイレクト 〔WDK-04〕"),
        (334, "st", "WDK-03", "グリーンテンタクル 〔WDK-03〕"),
        (335, "st", "WDK-02", "ブルーカタルシス 〔WDK-02〕"),
        (336, "st", "WDK-01", "レッドドーピング 〔WDK-01〕"),
        (510, "st", "WXD-23", "ブルーコンフレーション 〔WXD-23〕"),
        (511, "st", "WXD-22", "ブラックコンフレーション 〔WXD-22〕"),
        (512, "st", "WXD-21", "レッドジョーカー〔WXD-21〕"),
        (513, "st", "WXD-20", "グリーンカンニング 〔WXD-20〕"),
        (514, "st", "WXD-19", "ブラックブラインド 〔WXD-19〕"),
        (515, "st", "WXD-18", "グリーンベルセルク 〔WXD-18〕"),
        (516, "st", "WXD-17", "レッドオーネスト 〔WXD-17〕"),
        (517, "st", "WXD-16", "ブルーペティション〔WXD-16〕"),
        (518, "st", "WXD-15", "レッドプロミス〔WXD-15〕"),
        (519, "st", "WXD-14", "ブラックデザイア　ムービーバージョン〔WXD-14〕"),
        (520, "st", "WXD-13", "ホワイトホープ　ムービーバージョン〔WXD-13〕"),
        (521, "st", "WXD-12", "グリーンドリーム〔WXD-12〕"),
        (522, "st", "WXD-11", "ブラックニード 〔WXD-11〕"),
        (523, "st", "WXD-10", "レッドホープ 〔WXD-10〕"),
        (524, "st", "WXD-09", "ホワイトプレイ 〔WXD-09〕"),
        (525, "st", "WXD-08", "ブラックウィル 〔WXD-08〕"),
        (526, "st", "WXD-07", "ブラッククレイヴ 〔WXD-07〕"),
        (527, "st", "WXD-06", "ブルーリクエスト 〔WXD-06〕"),
        (528, "st", "WXD-05", "ブラックデザイア 〔WXD-05〕"),
        (529, "st", "WXD-04", "グリーンワナ 〔WXD-04〕"),
        (530, "st", "WXD-03", "ブルーアプリ 〔WXD-03〕"),
        (531, "st", "WXD-02", "レッドアンビション 〔WXD-02〕"),
        (532, "st", "WXD-01", "ホワイトホープ 〔WXD-01〕"),
        (20, "bo", "WX25-CP1", "ブルーアーカイブ SELECTOR"),
        (21, "bo", "WX24-P4", "FORTH SELECTOR"),
        (22, "bo", "WX24-P3", "REVERSAL SELECTOR"),
        (23, "bo", "WX24-P2", "loth SELECTOR"),
        (24, "bo", "WX24-P1", "RECOLLECT SELECTOR"),
        (25, "bo", "WXDi-P16", "LEGENDARY DIVA"),
        (26, "bo", "WXDi-P15", "DIVISIONS DIVA"),
        (27, "bo", "WXDi-CP02", "ブルーアーカイブ DIVA"),
        (28, "bo", "WXDi-P14", "フェゾーネ DIVA with 電音部"),
        (29, "bo", "WXDi-P13", "CONCORD DIVA"),
        (30, "bo", "WXDi-P12", "DISSONANCE DIVA"),
        (31, "bo", "WXDi-CP01", "にじさんじ DIVA"),
        (32, "bo", "WXDi-P11", "REUNION DIVA"),
        (33, "bo", "WXDi-P10", "PRISMATIC DIVA"),
        (34, "bo", "WXDi-P09", "CONFLATED DIVA"),
        (35, "bo", "WXDi-P08", "SPREAD DIVA"),
        (36, "bo", "WXDi-P07", "WELCOME BACK DIVA ～Lostorage～"),
        (37, "bo", "WXDi-P06", "WELCOME BACK DIVA ～selector～"),
        (38, "bo", "WXDi-P05", "CURIOSITY DIVA"),
        (39, "bo", "WXDi-P04", "VERTEX DIVA"),
        (40, "bo", "WXDi-P03", "STANDUP DIVA"),
        (41, "bo", "WXDi-P02", "CHANGING DIVA"),
        (42, "bo", "WXDi-P01", "GLOWING DIVA"),
        (43, "bo", "WXDi-P00", "INTERLUDE DIVA"),

        (50, "pr", "promotion", "プロモーションカード"),
        (61, "sp", "SPDi01", "スペシャルカード01"),
        (62, "sp", "SPDi02", "スペシャルカード02"),
        (63, "sp", "SPDi03", "スペシャルカード03"),
        (64, "sp", "SPDi04", "スペシャルカード04"),
        (65, "sp", "SPDi05", "スペシャルカード05"),
        (66, "sp", "SPDi06", "スペシャルカード06"),
        (67, "sp", "SPDi07", "スペシャルカード07"),
        (68, "sp", "SPDi08", "スペシャルカード08"),
        (69, "sp", "SPDi09", "スペシャルカード09"),
        (70, "sp", "SPDi10", "スペシャルカード10"),
        (71, "sp", "SPDi11", "スペシャルカード11"),
        (72, "sp", "SPDi12", "スペシャルカード12"),
        (73, "sp", "SPDi13", "スペシャルカード13"),
        (74, "sp", "SPDi14", "スペシャルカード14"),
        (75, "sp", "SPDi15", "スペシャルカード15"),
        (76, "sp", "SPDi16", "スペシャルカード16"),
        (77, "sp", "SPDi17", "スペシャルカード17"),
        # (78, "sp", "SPDi18", "スペシャルカード18"),
        (79, "sp", "SPDi19", "スペシャルカード19"),
        (80, "sp", "SPDi20", "スペシャルカード20"),
        (81, "sp", "SPDi21", "スペシャルカード21"),
        # (82, "sp", "SPDi22", "スペシャルカード22"),
        (83, "sp", "SPDi23", "スペシャルカード23"),
        (84, "sp", "SPDi24", "スペシャルカード24"),
        (85, "sp", "SPDi25", "スペシャルカード25"),
        (86, "sp", "SPDi26", "スペシャルカード26"),
        (87, "sp", "SPDi27", "スペシャルカード27"),
        (88, "sp", "SPDi28", "スペシャルカード28"),
        (89, "sp", "SPDi29", "スペシャルカード29"),
        (90, "sp", "SPDi30", "スペシャルカード30"),
        (91, "sp", "SPDi31", "スペシャルカード31"),
        (92, "sp", "SPDi32", "スペシャルカード32"),
        (93, "sp", "SPDi33", "スペシャルカード33"),
        (94, "sp", "SPDi34", "スペシャルカード34"),
        (95, "sp", "SPDi35", "スペシャルカード35"),
        (96, "sp", "SPDi36", "スペシャルカード36"),
        (97, "sp", "SPDi37", "スペシャルカード37"),
        (98, "sp", "SPDi38", "スペシャルカード38"),
        (99, "sp", "SPDi39", "スペシャルカード39"),

        (101, "bo", "WXK-11", "リンカーネイション"),
        (102, "bo", "WXK-10", "コリジョン"),
        (401, "bo", "WXEX-2", "アンブレイカブルセレクター"),
        (103, "bo", "WXK-09", "ディセンブル"),
        (104, "bo", "WXK-08", "アンリアリスティック"),
        (105, "bo", "WXK-07", "エクスプロード"),
        (106, "bo", "WXK-06", "オルタナティブ"),
        (402, "bo", "WXEX-1", "アンリミテッドセレクター"),
        (107, "bo", "WXK-05", "レトリック"),
        (108, "bo", "WXK-04", "ワイルズ"),
        (109, "bo", "WXK-03", "ユートピア"),
        (110, "bo", "WXK-02", "フルスクラッチ"),
        (111, "bo", "WXK-01", "クラクション"),
        (402, "bo", "WX-22", "アンロックドセレクター"),
        (403, "bo", "WX-21", "ビトレイドセレクター"),
        (404, "bo", "WX-20", "コネクテッドセレクター"),
        (405, "bo", "WX-19", "アンソルブドセレクター"),
        (406, "bo", "WX-18", "コンフレーテッド セレクター"),
        (407, "bo", "WX-17", "エクスポーズド セレクター"),
        (408, "bo", "WX-16", "ディサイデッド セレクター"),
        (409, "bo", "WX-15", "インサイテッド セレクター"),
        (410, "bo", "WX-14", "サクシードセレクター"),
        (411, "bo", "WX-13", "アンフェインドセレクター"),
        (412, "bo", "WX-12", "リプライドセレクター"),
        (413, "bo", "WX-11", "ディストラクテッドセレクター"),
        (414, "bo", "WX-10", "チェインドセレクター"),
        (415, "bo", "WX-09", "リアクテッドセレクター"),
        (416, "bo", "WX-08", "インキュベイトセレクター"),
        (417, "bo", "WX-07", "ネクストセレクター"),
        (418, "bo", "WX-06", "フォーチュンセレクター"),
        (419, "bo", "WX-05", "ビギニングセレクター"),
        (420, "bo", "WX-04", "インフェクテッドセレクター"),
        (421, "bo", "WX-03", "スプレッドセレクター"),
        (422, "bo", "WX-02", "ステアード セレクター"),
        (423, "bo", "WX-01", "サーブドセレクター"),
    )
    products = Product.objects.all()
    for prod in products_source:
        print(prod)
        p = products.filter(product_code=prod[2])

        if p.count() == 0:
            new_product = Product(product_code=prod[2], name=prod[3], product_type=prod[1], sort_asc=prod[0])
            new_product.save()
        else:
            target_product = Product.objects.get(product_code=prod[2])
            target_product.name = prod[3]
            target_product.sort_asc = prod[0]
            target_product.save()
    print('product update complete.')
