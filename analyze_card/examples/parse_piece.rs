use analyze_card::wixoss::{Piece, WixossCard};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WXDi-P14-001</p>
                                <p class="cardName">スプラッシュフィールド<br class="sp"><span>＜スプラッシュフィールド＞</span></p>
                                <div class="cardRarity">LR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXDi/WXDi-P14-001.jpg">
                                                                <p>Illust <span>Hitoto*</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>ピース</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>-</dd>

                                    <dt>色</dt>
                                    <dd>白</dd>

                                    <dt>レベル</dt>
                                    <dd>-</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>《白》×１</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>-</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>-</dd>
                                    <!-- コイン -->
                                    <dt>使用タイミング</dt>
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
                                        （<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_team.png" height="23" alt="【チーム】" />または<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_dreamteam.png" height="23" alt="【ドリームチーム】" />を持つピースはルリグデッキに合計１枚までしか入れられない）<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_terms_use.png" height="23" alt="【使用条件】" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_dreamteam.png" height="23" alt="【ドリームチーム】" />合計３種類以上の色を持つ（あなたの場にいるルリグ３体がこの条件を満たす）<br />
<br />
以下の２つから１つを選ぶ。<br />
①あなたのデッキの上からカードを５枚見る。その中からカードを２枚まで手札に加え、残りを好きな順番でデッキの一番下に置く。【シグニバリア】１つを得る。<br />
②対戦相手のシグニ１体を対象とし、それをトラッシュに置く。【ルリグバリア】１つを得る。                                    </div>

                                                                    <div class="cardText mb20">
                                        「3人で最高のパフォーマンスにしてみせる！」～アキノ～                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>【シグニバリア】や【ルリグバリア】は、アタックだけではなく効果によるダメージも防ぎますか？</dt>
                                                <dd>
                                                    はい、例えば【ルリグバリア】であればルリグによる「ダメージを与える効果」も防ぐことができます(《頂点へ一歩　ヒラナ》など)。ただし「ライフクロスをクラッシュする」効果は防ぐことができませんのでご注意ください。                                                </dd>
                                                                                            <dt>【シグニバリア】や【ルリグバリア】を複数得ることはできますか？</dt>
                                                <dd>
                                                    はい、得る効果が複数あればその分得られ、例えば【シグニバリア】を２つ持つこともあります。持つことのできる数に上限はありません。1回のシグニからのダメージで消費される【シグニバリア】は１つですので、この場合シグニからのダメージを2回防ぐことができます。                                                </dd>
                                                                                            <dt>【シグニバリア】や【ルリグバリア】は、ダメージを受ける際に消費しないことは選べますか？</dt>
                                                <dd>
                                                    いいえ、選べません。ダメージを受ける場合は強制で消費されます。ただし、他にもダメージを受けなくなる効果（《バン＝ダカーポ》の出現時能力など）がある場合、どれによって防ぐかは選ぶことができます。                                                </dd>
                                                                                            <dt>【ダブルクラッシュ】を持つシグニのアタックのダメージは【シグニバリア】１つで防げますか？</dt>
                                                <dd>
                                                    はい、防げます。【ダブルクラッシュ】はアタックによってダメージを与える場合にクラッシュする枚数が2枚になるという効果であり、ダメージを与える回数自体は1回です。【シグニバリア】はその1回を防ぐことができます。                                                </dd>
                                                                                            <dt>対戦相手のシグニの【Ｓランサー】は、【シグニバリア】で防げますか？</dt>
                                                <dd>
                                                    あなたのライフクロスが無い場合、【Ｓランサー】でダメージを与える効果は【シグニバリア】で防げます。あなたのライフクロスがある場合、【Ｓランサー】は【ランサー】同様にライフクロスをクラッシュするという効果ですので防ぐことができません。                                                </dd>
                                                                                            <dt>【シグニバリア】や【ルリグバリア】は、ルリグが能力を失ったら無くなりますか？</dt>
                                                <dd>
                                                    いいえ、ルリグが得ている能力ではありませんので無くなりません。                                                </dd>
                                                                                            <dt>②の効果を選んだ際、対戦相手のシグニが1体もなくても【ルリグバリア】を得られますか？</dt>
                                                <dd>
                                                    はい、得られます。前半の効果の結果に関係なく、後半の効果を処理できます。                                                </dd>
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

    let piece = Piece::from_source(source);
    // let card: Card = piece.into();
    // println!("{}", Into::<Card>::into(piece));
    println!("{}", &piece);
}
