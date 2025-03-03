use webapp::analyze::wixoss::{Card};

fn main() {
    let text = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WX22-025</p>
                                <p class="cardName">天空の主神　ゼウシアス<br class="sp"><span>＜テンクウノシュシンゼウシアス＞</span></p>
                                <div class="cardRarity">SR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WX22/WX22-025.jpg">
                                                                <p>Illust <span>Hitoto*</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>シグニ</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>精像：天使</dd>

                                    <dt>色</dt>
                                    <dd>無</dd>

                                    <dt>レベル</dt>
                                    <dd>5</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>-</dd>

                                    <dt>リミット</dt>
                                    <dd>10</dd>

                                    <dt>パワー</dt>
                                    <dd>15000</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>タウィル限定</dd>
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
                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_regular.png" height="23" alt="【常】" />：【マルチエナ】<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_regular.png" height="23" alt="【常】" />：【シャドウ】<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_regular.png" height="23" alt="【常】" />：このシグニはすべての色を得る。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_auto.png" height="23" alt="【自】" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_turn_01.png" height="23" alt="《ターン１回》" />：このシグニがアタックしたとき、対戦相手が、対象の自分のシグニ１体を場からトラッシュに置くか、自分の手札を２枚捨てるか、対象の自分のエナゾーンからカード３枚をトラッシュに置かないかぎり、対戦相手にダメージを与える。                                    </div>

                                                                    <div class="cardText mb20">
                                        全知全能を司る唯一無二の神。                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>リミット消費とは何ですか？</dt>
                                                <dd>
                                                    《天空の主神　ゼウシアス》のレベルは５ですが、場に出すことのできるシグニのレベルの合計であるリミットは１０として扱われます。例えばリミット１２のルリグの場合、このシグニのみが場にあると他に場に出せるシグニはレベル２を１体かレベル１を２体です。逆にリミット１２のルリグの場合に場にレベル３のシグニがある場合、《天空の主神　ゼウシアス》を場に出すことはできません。                                                </dd>
                                                                                            <dt>【シャドウ】とは何ですか？</dt>
                                                <dd>
                                                    【シャドウ】を持つあなたのシグニは、対戦相手の、能力と効果によって対象とされず、例えば対戦相手の「シグニ1体を対象とし、それをバニッシュする。」といった効果によっては対象とされません。<br />
逆に、「すべてのシグニをバニッシュする」といった効果の影響は受けます。<br />
<br />
<a class="news_link" href="https://www.takaratomy.co.jp/products/wixoss/faq/glossary.php" target="_blank">用語集【シャドウ】</a>をご参照ください。                                                </dd>
                                                                                            <dt>「このシグニはすべての色を得る」というのは手札やトラッシュでも有効ですか？</dt>
                                                <dd>
                                                    いいえ、このシグニが場にあるときにのみ、このシグニはすべての色を得ます。手札やトラッシュでは無色のシグニのままです。                                                </dd>
                                                                                            <dt>《天空の主神　ゼウシアス》と《白羅星　ウラヌス》が場にある場合、このシグニの色はどうなりますか？</dt>
                                                <dd>
                                                    《白羅星　ウラヌス》が先に場に出ている場合は先に白となってからすべての色を得ます。《白羅星　ウラヌス》が後から出た場合は、すべての色を得ていた《天空の主神　ゼウシアス》が上書きされて白のみとなります。                                                </dd>
                                                                                            <dt>自動能力は、対戦相手の手札が１枚の場合、それを捨てることを選べますか？</dt>
                                                <dd>
                                                    いいえ、２枚捨てることができない場合、それを選ぶことはできません。他の選択肢も、トラッシュに置く枚数が満たない場合はそれを選ぶことはできず、どれもできない場合はダメージを与えられます。                                                </dd>
                                                                                            <dt>自動能力は、できる場合でもどれもトラッシュに置かずにダメージを受けることを選べますか？</dt>
                                                <dd>
                                                    はい、どれも行わずにダメージを受けることも選べます。                                                </dd>
                                                                                            <dt>自動能力で、《天空の主神　ゼウシアス》の正面のシグニがトラッシュに置かれた場合はアタックはどうなりますか？</dt>
                                                <dd>
                                                    まず「アタックしたとき」の能力が発動し、その後に他のトリガー能力等が無ければ正面のシグニとのバトルや対戦相手へのダメージとなりますので、この場合はアタックにより対戦相手へダメージを与えます。                                                </dd>
                                                                                            <dt>《天空の主神　ゼウシアス》の自動能力について以下のような場合は自動能力でダメージをプレイヤーに与えることはできますか？<br />
①「効果を受けない」を持つシグニをトラッシュに置こうとした場合<br />
②【シャドウ】を持つシグニトラッシュに置こうとした場合<br />
③レゾナをトラッシュに置こうとした場合</dt>
                                                <dd>
                                                    ①「効果を受けない」を持つシグニは移動することができないので、トラッシュに置くことができません。そのため、この自動能力でダメージが与えられます。<br />
②【シャドウ】を持つシグニはこの能力で対象とすることができません。そのため、他のシグニを対象としトラッシュに置くか、他の選択肢を選ばない限り、この自動能力でダメージが与えられます。<br />
③《天空の主神　ゼウシアス》のような「Aしないかぎり、Bする」というテキストはAの部分を他の行動で置き換えて実行すると、Bは実行されません。レゾナはトラッシュに置かれる場合、代わりにルリグデッキに戻るため、この自動能力ではダメージが与えません。                                                </dd>
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

"#;

    let t = Card::detect_card_type(&String::from(text));
    let c = Card::card_from_html(&String::from(text));
    println!("{}", t);
    println!("{}", c.unwrap());
}