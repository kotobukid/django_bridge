use analyze_card::wixoss::{Card, CardType, SpellCraft, WixossCard};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WXDi-P14-TK01</p>
                                <p class="cardName">フェゾーネマジック・ホワイト<br class="sp"><span>＜フェゾーネマジックホワイト＞</span></p>
                                <div class="cardRarity">TK</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXDi/WXDi-P14-TK01.jpg">
                                                                <p>Illust <span>かにゃぴぃ</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>スペル<br />
クラフト</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>-</dd>

                                    <dt>色</dt>
                                    <dd>白</dd>

                                    <dt>レベル</dt>
                                    <dd>-</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>《無》×１</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>-</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>-</dd>
                                    <!-- コイン -->
                                    <dt>ガード</dt>
                                    <dd>-</dd>

                                    <dt>フォーマット</dt>
                                    <dd></dd>

                                    <!-- 0205mao -->
                                    <!-- 0205niimura -->
                                    <dt>ストーリー</dt>
                                    <dd>
                                                                            -
                                                                        </dd>
                                </dl>

                                                                    <div class="cardSkill">
                                        （このスペルはあなたのメインフェイズにルリグデッキから使用できる）<br />
（クラフトであるスペルは、使用後にゲームから除外される）<br />
<br />
あなたのトラッシュから<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_guard_mini.png" height="23" alt="《ガードアイコン》" />を持つシグニ１枚を対象とし、それを手札に加える。                                    </div>

                                                                    <div class="cardText mb20">
                                        「わ、わたしだって…えい！」～アキノ～                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>スペル/クラフトとは何ですか？</dt>
                                                <dd>
                                                    効果によってあなたのルリグデッキに加えられるスペルです。あなたのメインフェイズに、ルリグデッキからチェックゾーンに置き、使用できます。通常のスペルの使用手順と同じように対戦相手は＜スペルカットイン＞を使用でき、スペルの使用コストが増減する効果の影響を受けます。                                                </dd>
                                                                                    </dl>
                                    </div>
                                                            </div>
                        </div>
                    </div>
                </section>

        </main><!-- .site-main -->
    </div><!-- .content-area -->

    <script>
        $(function() {
            // //サブメニューナビゲーション
            // $('.accordionTrg').click(function () {
            //     $('.accordion').slideToggle();
            //     console.log('detail.php');
            //     $(this).toggleClass('opn');
            // });
            // //チェックすべて外す
            // $('#noncheck').click(function () {
            //     $('.cardform input[type="checkbox"]').prop('checked', false);
            // });
            /*
            $('.cboxElement').click(function () {
              $('.mordal').css('display', 'block');
              $('body,html').css('overflow', 'hidden');
            });*/
            $('.mordal .close').click(function () {
                /*$('.mordal').css('display', 'none');
                $('body,html').css('overflow', 'auto');*/
                parent.$.fn.colorbox.close(); return false;
                //console.log("ここ");
            });
        });
    </script>

    <!-- /新デザイン -->
    </body>
    </html>

"#.into();

    let spell = SpellCraft::from_source(source);
    println!("{}", &spell);
    let card: Card = spell.into();
    // println!("{}", card);

    assert_eq!(card.card_type, CardType::SpellCraft);
}
