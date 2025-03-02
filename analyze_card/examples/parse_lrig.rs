use analyze_card::wixoss::{Card, card_type::CardType, Lrig, WixossCard};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WXDi-P14-006</p>
                                <p class="cardName">炎泳華　遊月・燦<br class="sp"><span>＜エンエイカユヅキサン＞</span></p>
                                <div class="cardRarity">LR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXDi/WXDi-P14-006.jpg">
                                                                <p>Illust <span>夕子</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>ルリグ</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>ユヅキ</dd>

                                    <dt>色</dt>
                                    <dd>赤</dd>

                                    <dt>レベル</dt>
                                    <dd>3</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>《赤》×２</dd>

                                    <dt>コスト</dt>
                                    <dd>-</dd>

                                    <dt>リミット</dt>
                                    <dd>6</dd>

                                    <dt>パワー</dt>
                                    <dd>-</dd>

                                    <!-- チーム -->
                                    <dt>チーム</dt>
                                    <dd>-</dd>
                                    <!-- コイン -->
                                    <dt>コイン</dt>
                                    <dd>-</dd>

                                    <dt>フォーマット</dt>
                                    <dd><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_format_key.png" height="23" alt="《キーアイコン》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_format_diva.png" height="23" alt="《ディーヴァアイコン》" /></dd>

                                    <!-- 0205mao -->
                                    <!-- 0205niimura -->
                                    <dt>ストーリー</dt>
                                    <dd>
                                                                            -
                                                                        </dd>
                                </dl>

                                                                    <div class="cardSkill">
                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_arrival.png" height="23" alt="【出】" />ライフクロス１枚をクラッシュする：対戦相手のライフクロス１枚をトラッシュに置く。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_starting.png" height="23" alt="【起】" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_turn_01.png" height="23" alt="《ターン１回》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_red.png" height="23" alt="《赤》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_null.png" height="23" alt="《無》" />：あなたのライフクロスが２枚以下の場合、あなたの赤のシグニ１体を対象とし、ターン終了時まで、それは【アサシン】を得る。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_starting.png" height="23" alt="【起】" />エクシード４：フェゾーネマジックのクラフトから２種類を１枚ずつ公開しルリグデッキに加える。（フェゾーネマジックは５種類ある）                                    </div>

                                                                    <div class="cardText mb20">
                                        「よーし！いっくぞー！」                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>フェゾーネマジックとは何ですか？</dt>
                                                <dd>
                                                    この効果によってゲーム外からルリグデッキに加えられるスペル/クラフトです。5種類あり、この効果ではそのうち2種類を選んで加えることができます。同じ種類を2枚加えることはできません。                                                </dd>
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

    let lrig = Lrig::from_source(source);
    println!("{}", &lrig);
    let card: Card = lrig.into();
    // println!("{}", card);

    assert_eq!(card.card_type, CardType::Lrig);
}
