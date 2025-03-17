use webapp::analyze::wixoss::{card::CardType, Card, card::ResonaCraft, WixossCard};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WXDi-P09-TK02A</p>
                                <p class="cardName">コードイート　セアブラマシマシ<br class="sp"><span>＜コードイートセアブラマシマシ＞</span></p>
                                <div class="cardRarity">TK</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXDi/WXDi-P09-TK02A.jpg">
                                                                <p>Illust <span>松本エイト</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>シグニ<br />
クラフト</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>奏械：調理</dd>

                                    <dt>色</dt>
                                    <dd>緑</dd>

                                    <dt>レベル</dt>
                                    <dd>2</dd>

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
                                        （このクラフトは効果以外によっては場に出せない）<br />
（【アクセ】はシグニ１体に１枚までしか付けられない。このクラフトが付いているシグニが場を離れるとこのクラフトはゲームから除外される）<br />
『<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_regular.png" height="23" alt="【常】" />：これにアクセされているシグニは【ランサー】を得る。<br />
（【ランサー】を持つシグニがバトルでシグニをバニッシュしたとき、対戦相手のライフクロスを１枚クラッシュする）』                                    </div>

                                                                    <div class="cardText mb20">
                                        「はーい、マシマシコール入りました♪」                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>これはどう取り扱えば良いですか？</dt>
                                                <dd>
                                                    これは《メル＝チアーズ》の効果でルリグデッキに加えられるシグニクラフトです。クラフトはカードとしては扱いませんので、ルリグデッキからカードを選ぶ効果（《紅将姫 リル//メモリア》）では選べません。                                                </dd>
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

    let resona = ResonaCraft::from_source(source);
    println!("{}", &resona);
    let card: Card = resona.into();
    assert_eq!(card.card_type, CardType::ResonaCraft);
}
