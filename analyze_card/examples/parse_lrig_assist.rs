use analyze_card::wixoss::{card_type::CardType, Card, LrigAssist, WixossCard};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WXDi-P14-026</p>
                                <p class="cardName">ミルルン☆キャッチ<br class="sp"><span>＜ミルルンキャッチ＞</span></p>
                                <div class="cardRarity">LC</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXDi/WXDi-P14-026.jpg">
                                                                <p>Illust <span>かにかま</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>アシストルリグ</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>ミルルン</dd>

                                    <dt>色</dt>
                                    <dd>青</dd>

                                    <dt>レベル</dt>
                                    <dd>2</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>《無》×３</dd>

                                    <dt>コスト</dt>
                                    <dd>-</dd>

                                    <dt>リミット</dt>
                                    <dd>1</dd>

                                    <dt>パワー</dt>
                                    <dd>-</dd>

                                    <!-- チーム -->
                                    <dt>チーム</dt>
                                    <dd>-</dd>
                                    <!-- コイン -->
                                    <dt>使用タイミング</dt>
                                    <dd>メインフェイズ<br />
アタックフェイズ</dd>

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
                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_arrival.png" height="23" alt="【出】" />：対戦相手のシグニを２体まで対象とし、それらをダウンする。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_arrival.png" height="23" alt="【出】" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_blue.png" height="23" alt="《青》" />：対戦相手の手札を１枚見ないで選び、捨てさせる。<br />
（<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_arrival.png" height="23" alt="【出】" />能力の：の左側はコストである。コストを支払わず発動しないことを選んでもよい）                                    </div>

                                                                    <div class="cardText mb20">
                                        「捕まえる～ん☆」                                    </div>

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

    let lrig = LrigAssist::from_source(source);
    println!("{}", &lrig);
    let card: Card = lrig.into();
    // println!("{}", card);

    assert_eq!(card.card_type, CardType::LrigAssist);
}
