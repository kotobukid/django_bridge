use analyze_card::wixoss::{card::CardType, Card, Signi, WixossCard};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WXDi-P14-040</p>
                                <p class="cardName">羅星姫　リメンバ//フェゾーネ<br class="sp"><span>＜ラセイキリメンバフェゾーネ＞</span></p>
                                <div class="cardRarity">SR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXDi/WXDi-P14-040.jpg">
                                                                <p>Illust <span>九十きゅうり</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>シグニ</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>奏羅：宇宙</dd>

                                    <dt>色</dt>
                                    <dd>白</dd>

                                    <dt>レベル</dt>
                                    <dd>3</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>-</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>10000</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>-</dd>
                                    <!-- コイン -->
                                    <dt>ガード</dt>
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
                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_auto.png" height="23" alt="【自】" />：このシグニがアタックしたとき、対戦相手の場に凍結状態のルリグとシグニが合計３体以上いる場合、<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_null.png" height="23" alt="《無》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_null.png" height="23" alt="《無》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_null.png" height="23" alt="《無》" />を支払ってもよい。そうした場合、ターン終了時まで、このシグニは【アサシン】を得る。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_arrival.png" height="23" alt="【出】" />：センタールリグではない対戦相手のルリグ１体を対象とし、それを凍結する。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_starting.png" height="23" alt="【起】" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_turn_01.png" height="23" alt="《ターン１回》" />アップ状態のシグニ１体をダウンする：対戦相手のシグニ１体を対象とし、それを凍結する。                                    </div>

                                                                    <div class="cardText mb20">
                                        「さあ、私と楽しみましょうよ～？」                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>起動能力のコストとして、対戦相手のアップ状態のシグニをダウンすることはできますか？</dt>
                                                <dd>
                                                    いいえ、できません。特に記載がない限り、コストとして支払えるのは自分側のカードのみとなります。                                                </dd>
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

    let signi = Signi::from_source(source);
    println!("{}", &signi);
    let card: Card = signi.into();
    // println!("{}", card);

    assert_eq!(card.card_type, CardType::Signi);
}
