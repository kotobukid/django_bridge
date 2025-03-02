use analyze_card::wixoss::{Card};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">PR-K075</p>
                                <p class="cardName">応諾の鍵主　ウムル（ウムルとタウィルその２ 付録）<br class="sp"><span>＜オウダクノカギヌシウムル＞</span></p>
                                <div class="cardRarity">PR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/PR/PR-K075.jpg">
                                                                <p>Illust <span>OYSTER</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>キー</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>-</dd>

                                    <dt>色</dt>
                                    <dd>黒</dd>

                                    <dt>レベル</dt>
                                    <dd>-</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>《コイン》×２<br />
《無》×３</dd>

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
                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_regular.png" height="23" alt="【常】" />：あなたのセンタールリグは以下の能力を得る。<br />
<div class="card_ability_add_border"><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_starting.png" height="23" alt="【起】" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_turn_01.png" height="23" alt="《ターン１回》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_phase_attack.png" height="23" alt="《アタックフェイズアイコン》" />エクシード１：対戦相手のシグニ１体を対象とし、ターン終了時まで、それのパワーを－7000する。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_starting.png" height="23" alt="【起】" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_turn_01.png" height="23" alt="《ターン１回》" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_phase_attack.png" height="23" alt="《アタックフェイズアイコン》" />エクシード２：対戦相手のシグニを２体まで対象とし、あなたのデッキの上からカードを９枚トラッシュに置く。この方法でカードが９枚トラッシュに置かれた場合、ターン終了時まで、それらのパワーを合わせて－18000する。この効果では1000単位でしか数字を割り振ることができない。<br />
</div><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_starting.png" height="23" alt="【起】" />このキーを場からルリグトラッシュに置く：あなたのトラッシュからシグニ１体を対象とし、それを手札に加える。                                    </div>

                                                                    <div class="cardText mb20">
                                        ワシを呼ぶなら呪文を唱えよ。かっこいいやつじゃぞ。～ウムル～                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>このキーを場に出す場合、どのようにコストを支払いますか？</dt>
                                                <dd>
                                                    まず、このキーを場に出すことを宣言します。その後、《コインアイコン》×２分のコインを得ているコインから支払い、《無》×３分のカードをエナゾーンからトラッシュに置きエナコストを支払います。そうすることで、このキーは場に出すことができます。                                                </dd>
                                                                                            <dt>《アイヤイ★ディール》の下段の常時能力でエナを支払うことで、このキーを場に出すことができますか？</dt>
                                                <dd>
                                                    はい、《アイヤイ★ディール》の下段の常時能力をこのキーのエナコストの支払いに使用することができます。                                                </dd>
                                                                                            <dt>デッキが８枚以下でエクシード２の能力を使用することはできますか？</dt>
                                                <dd>
                                                    はい、使用することができます。この場合、デッキのカードをすべてトラッシュに置き、「この方法で～」以降の処理は実行されず、この処理が完了した後にリフレッシュを行います。                                                </dd>
                                                                                            <dt>エクシード２の能力は相手のシグニが１体の場合でも使用することができますか？</dt>
                                                <dd>
                                                    はい、１体しかない場合でも使用できます。このテキストのように２枚「まで～」と書かれている場合には０～２枚まで好きな数のシグニを対象とすることができるためです。                                                </dd>
                                                                                            <dt>対戦相手の場にある２体のシグニのパワーが合計15000しかない場合、エクシード２の能力を使用して２体のシグニをバニッシュできますか？</dt>
                                                <dd>
                                                    はい、パワー以上に数値を割り振ることは可能です。                                                </dd>
                                                                                            <dt>エクシード２の能力を使用した後、《ゲット・グロウ》などでルリグがグロウしました。エクシード２の能力はもう一度使用できますか？</dt>
                                                <dd>
                                                    はい、可能です。ターン１回とは同一のカードがその能力そのものを使用・発動する回数を制限するものです。キーでターン1回という制限を持つ能力を得ているルリグがその能力使用後、《ゲット・グロウ》などでグロウすると、他のルリグでありそのルリグの持つ能力となりますので、再度ターン1回の能力を使用することができます。                                                </dd>
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
    let card = Card::card_from_html(&source);
    println!("{}", serde_json::to_string_pretty(&card).unwrap());
}