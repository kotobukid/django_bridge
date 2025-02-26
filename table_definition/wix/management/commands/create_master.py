from linecache import updatecache

from django.core.management.base import BaseCommand
from wix.models import Color, CardType, Lrig, Klass, Timing


class Command(BaseCommand):
    help = ""

    def handle(self, *args, **options):
        update_color()
        update_card_type()
        update_lrig()
        update_klass()
        update_timing()


def update_color():
    color_source = (
        ('w', '白', 0),
        ('r', '赤', 1),
        ('u', '青', 2),
        ('g', '緑', 3),
        ('k', '黒', 4),
        ('l', '無', 5),
    )
    colors_existing = Color.objects.all()
    for color in color_source:
        print(color)
        c_ex = colors_existing.filter(code=color[0])

        if c_ex.count() == 0:
            new_color = Color(code=color[0], name=color[1], sort_asc=color[2])
            new_color.save()
        else:
            if c_ex[0].name != color[1]:
                target_color = Color.objects.get(code=color[0])
                target_color[0].name = color[1]
                target_color[0].save()
    print('color update complete.')


def update_card_type():
    type_source = (
        ('lrig', 'ルリグ', 0),
        ('arts', 'アーツ', 1),
        ('piece', 'ピース', 2),
        ('signi', 'シグニ', 3),
        ('spell', 'スペル', 4),
        ('resona', 'レゾナ', 5),
        ('token', 'トークン', 100),
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
        ('main', 'メインフェイズ', 0),
        ('attack', 'アタックフェイズ', 1),
        ('spellcutin', 'スペルカットイン', 2),
    )
    timing_existing = Timing.objects.all()
    for timing in timing_source:
        print(timing)
        t_ex = timing_existing.filter(code=timing[0])

        if t_ex.count() == 0:
            new_timing = Timing(code=timing[0], name=timing[1], sort_asc=timing[2])
            new_timing.save()
        else:
            if t_ex[0].name != timing[1] or t_ex[0].sort_asc != timing[2]:
                target_timing = Timing.objects.get(code=timing[0])
                target_timing[0].name = timing[1]
                target_timing[0].sort_asc = timing[2]
                target_timing[0].save()
    print('timing update complete.')
