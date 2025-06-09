use webapp::analyze::wixoss::{card::Key, WixossCard};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">PR-K060</p>
                                <p class="cardName">虚鍵の閻魔　ウリス（ウィクロスカード大全Ｋ-Ⅲ 付録）<br class="sp"><span>＜キョケンノエンマウリス＞</span></p>
                                <div class="cardRarity">PR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/PR/PR-K060.jpg">
                                                                <p>Illust <span>しおぼい</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>キー</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>-</dd>

                                    <dt>色</dt>
                                    <dd>無</dd>

                                    <dt>レベル</dt>
                                    <dd>-</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>《コイン》×１</dd>

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
                                    <dd><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_format_key.png" height="23" alt="《キーアイコン》" /></dd>

                                    <!-- 0205mao -->
                                    <!-- 0205niimura -->
                                    <dt>ストーリー</dt>
                                    <dd>
                                                                            -
                                                                        </dd>
                                </dl>

                                                                    <div class="cardSkill">
                                        このキーはあなたのエナゾーンにあるカードが持つ色が合計３種類以上ある場合にしか新たに場に出せない。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_regular.png" height="23" alt="【常】" />：あなたのセンタールリグは以下の能力を得る。<br />
<div class="card_ability_add_border"><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_starting.png" height="23" alt="【起】" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_phase_attack.png" height="23" alt="《アタックフェイズアイコン》" />エクシード４：あなたの手札を１枚選ぶ。対戦相手は<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_white2.png" height="23" alt="《白2》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_red2.png" height="23" alt="《赤2》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_blue2.png" height="23" alt="《青2》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_green2.png" height="23" alt="《緑2》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_black2.png" height="23" alt="《黒2》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_null2.png" height="23" alt="《無2》" />から１つを宣言する。そのカードを公開し、それが宣言されたアイコンを持つカードではない場合、対戦相手のすべてのシグニをトラッシュに置く。<br />
</div><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_regular.png" height="23" alt="【常】" />：あなたは限定条件を無視してアーツを使用できる。                                    </div>

                                                                    <div class="cardText mb20">
                                        放ちたくてたまらない…！                                    </div>

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

    let key = Key::from_source(source);
    // let card: Card = piece.into();
    // println!("{}", Into::<Card>::into(piece));
    println!("{}", &key);
}
