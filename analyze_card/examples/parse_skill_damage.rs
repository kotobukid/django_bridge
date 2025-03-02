use analyze_card::wixoss::{card::CardType, Card, Spell, WixossCard};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WX11-042</p>
                                <p class="cardName">西部の銃声<br class="sp"><span>＜セイブノジュウセイ＞</span></p>
                                <div class="cardRarity">R</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WX11/WX11-042.jpg">
                                                                <p>Illust <span>紅緒</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>スペル</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>-</dd>

                                    <dt>色</dt>
                                    <dd>赤</dd>

                                    <dt>レベル</dt>
                                    <dd>-</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>《赤》×５</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>-</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>花代限定</dd>
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
                                        対戦相手にダメージを与える。（対戦相手のライフクロスが１枚以上ある場合、ライフクロス１枚をクラッシュし、０枚の場合、あなたはゲームに勝利する）                                    </div>
                                                                                                    <div class="cardSkill">
                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_burst.png" width="26" height="24" alt="ライフバースト" />：<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_red.png" height="23" alt="《赤》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_red.png" height="23" alt="《赤》" />を支払ってもよい。そうした場合、対戦相手のライフクロス１枚をクラッシュする。                                    </div>

                                                                    <div class="cardText mb20">
                                        動くな！もう撃ったけど！～花代～                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>相手のライフクロスが0枚のとき、《西部の銃声》のライフバーストが発動しました。《赤》《赤》を支払った場合、ゲームに勝利できますか？</dt>
                                                <dd>
                                                    いいえ、《西部の銃声》のライフバーストは、スペルとして使用したときの効果とは違い「ライフクロス1枚をクラッシュする」という効果となっております。ダメージを与えているわけではないためゲームに勝利するわけではなく、ライフクロスが0枚の場合はそのまま効果が終了します。                                                </dd>
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

    let spell = Spell::from_source(source);
    println!("{}", &spell);
    let card: Card = spell.into();
    // println!("{}", card);

    assert_eq!(card.card_type, CardType::Spell);
}
