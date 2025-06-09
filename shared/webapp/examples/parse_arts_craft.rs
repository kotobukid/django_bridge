use webapp::analyze::wixoss::{card::ArtsCraft, card::CardType, Card, WixossCard};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WXK03-TK-01B</p>
                                <p class="cardName">落華流粋<br class="sp"><span>＜レクイエム＞</span></p>
                                <div class="cardRarity">-</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXK03/WXK03-TK-01B.jpg">
                                                                <p>Illust <span>村上ゆいち</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>アーツ<br />
クラフト</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>-</dd>

                                    <dt>色</dt>
                                    <dd>赤緑</dd>

                                    <dt>レベル</dt>
                                    <dd>-</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>《赤》×１<br />
《緑》×１<br />
《無》×１</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>-</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>-</dd>
                                    <!-- コイン -->
                                    <dt>ガード</dt>
                                    <dd>メインフェイズ<br />
アタックフェイズ</dd>

                                    <dt>フォーマット</dt>
                                    <dd><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_format_key.png" height="23" alt="《キーアイコン》" /></dd>

                                    <!-- 0205mao -->
                                    <!-- 0205niimura -->
                                    <dt>ストーリー</dt>
                                    <dd>
                                                                            -
                                                                        </dd>
                                </dl>

                                                                    <div class="cardSkill">
                                        以下の４つから１つを選ぶ。あなたのセンタールリグが＜リル＞か＜メル＞の場合、代わりに２つまで選ぶ。<br />
①対戦相手のパワー12000以下のシグニ１体を対象とし、それをバニッシュする。<br />
②対戦相手のパワー12000以上のシグニ１体を対象とし、それをバニッシュする。<br />
③あなたのシグニ１体を対象とし、ターン終了時まで、それは【ダブルクラッシュ】を得る。そのシグニがレベル４以上の場合、追加でそれは【アサシン】を得る。<br />
④あなたのデッキの一番上のカードをライフクロスに加える。手札を２枚捨てる。                                    </div>

                                                                    <div class="cardText mb20">
                                        これが、俺のレクイエムだよ……！～カーニバル～                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>＜リル＞か＜メル＞で使用した場合、同じ選択肢を2回以上選べますか？</dt>
                                                <dd>
                                                    いいえ、同じ選択肢を複数回選ぶことはできません。                                                </dd>
                                                                                            <dt>対戦相手にパワー１２０００のシグニがある場合、①と②の効果で使用し両方ともそのパワー１２０００のシグニ１体を対象とすることはできますか？</dt>
                                                <dd>
                                                    はい、可能です。                                                </dd>
                                                                                            <dt>手札が１枚以下のときでも、④の効果を選べますか？</dt>
                                                <dd>
                                                    はい、可能です。あなたのデッキの一番上のカードをライフクロスに加えた後、手札が１枚以下の場合は残っている手札を捨てます。                                                </dd>
                                                                                            <dt>④の効果で、自分のレベル４のシグニに【ダブルクラッシュ】と【アサシン】を与えた後に何らかの効果でそのシグニのレベルが下がった場合、【アサシン】は失われますか？</dt>
                                                <dd>
                                                    いいえ、《落華流粋》で得た【アサシン】は、その後にそのシグニのレベルが下がったとしてもターン終了時までは失いません。                                                </dd>
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

    let arts = ArtsCraft::from_source(source);
    println!("{}", &arts);
    let card: Card = arts.into();
    // println!("{}", card);

    assert_eq!(card.card_type, CardType::ArtsCraft);
}
