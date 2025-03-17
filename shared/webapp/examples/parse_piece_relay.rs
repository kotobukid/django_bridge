use webapp::analyze::wixoss::{card::PieceRelay, WixossCard};

fn main() {
    let source: String = r#"

    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WXDi-CP01-001</p>
                                <p class="cardName">世怜音女学院　After School　<br class="sp"><span>＜セレイネジョガクインアフタースクール＞</span></p>
                                <div class="cardRarity">LR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXDi/WXDi-CP01-001.jpg">
                                                                <p>Illust <span>林けゐ</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>ピース<br />
リレー</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>-</dd>

                                    <dt>色</dt>
                                    <dd>白</dd>

                                    <dt>レベル</dt>
                                    <dd>-</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>《無》×０</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>-</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>-</dd>
                                    <!-- コイン -->
                                    <dt>ガード</dt>
                                    <dd>メインフェイズ</dd>

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
                                        このピースを使用する際、使用コストとして追加でエクシード４を支払ってもよい。（あなたのルリグの下からカードを合計４枚ルリグトラッシュに置く）<br />
<br />
あなたのデッキの上からカードを５枚見る。その中から＜バーチャル＞のシグニを２枚まで公開し手札に加え、残りを好きな順番でデッキの一番下に置く。追加でエクシード４を支払っていた場合、【エナチャージ１】をする。                                    </div>

                                                                    <div class="cardText mb20">
                                        創立から優に100年を超える、由緒ある学校「世怜音女学院」。                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>リレーピースとは何ですか？</dt>
                                                <dd>
                                                    ピースの1種です。単体では通常のピースと変わらずリレーであることに意味はありませんが、リレーピースを使用することで使用できるようになるピースが存在します。                                                </dd>
                                                                                            <dt>エクシード４のコストはどう支払えばいいですか？</dt>
                                                <dd>
                                                    センタールリグとアシストルリグ含めてあなたのルリグの下から合計4枚をルリグトラッシュに置くことで、コストを支払うことができます。                                                </dd>
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

    let piece = PieceRelay::from_source(source);
    // let card: Card = piece.into();
    println!("{}", piece);
}