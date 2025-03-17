use webapp::analyze::wixoss::{card::CardType, Card};

fn main() {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">SP32-024</p>
                                <p class="cardName">黒幻蟲　アラクネ・パイダ（セレクターセレクション）<br class="sp"><span>＜コクゲンチュウアラクネパイダ＞</span></p>
                                <div class="cardRarity">SP</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/SP32/SP32-024.jpg">
                                                                <p>Illust <span>九鳥ぱんや</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>レゾナ</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>精生：凶蟲</dd>

                                    <dt>色</dt>
                                    <dd>黒</dd>

                                    <dt>レベル</dt>
                                    <dd>4</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>-</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>15000</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>ミュウ限定</dd>
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
                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_AppearanceCond.png" height="23" alt="【出現条件】" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_phase_main.png" height="23" alt="《メインフェイズアイコン》" />レゾナではない＜凶蟲＞のシグニ２体をあなたの場からトラッシュに置く<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_regular.png" height="23" alt="【常】" />：対戦相手は【チャーム】が付いているシグニの<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_starting.png" height="23" alt="【起】" />能力を使用できない。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_auto.png" height="23" alt="【自】" />：対戦相手のシグニ１体が場に出たとき、対戦相手は自分のデッキの一番上のカードをそのシグニの【チャーム】にする。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_auto.png" height="23" alt="【自】" />：各アタックフェイズ開始時、対戦相手は【チャーム】が付いている自分のシグニ１体を対象とし、それをバニッシュする。                                    </div>

                                                                    <div class="cardText mb20">
                                        イトアート、キレイデショ。～アラクネ・パイダ～                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>アタックフェイズ開始時に発動する自動能力より先にアーツを使用し、《黒幻蟲　アラクネ・パイダ》をバニッシュすることは可能ですか？</dt>
                                                <dd>
                                                    いいえ、《黒幻蟲　アラクネ・パイダ》の下段自動能力はプレアタックステップに入ったときに発動しますが、それが処理されるまでアーツ等を使用することは出来ません。                                                </dd>
                                                                                            <dt>対戦相手が《不可解な誇超　コンテンポラ》を場に出した場合、中段自動能力で【チャーム】を付けることができますか？</dt>
                                                <dd>
                                                    はい、可能です。《不可解な誇超　コンテンポラ》の「効果を受けない」という能力は<br />
「カードの状態変化、数値とテキストの変化、効果による移動」を受けないということですが、【チャーム】はこのいずれにも該当しない為、付けることができます。                                                </dd>
                                                                                            <dt>《黒幻虫　アラクネ・パイダ》が場にある状態で、その正面に《コードラビリンス　ルーブル》が出た場合、《コードラビリンス　ルーブル》に【チャーム】を付けることは出来ますか？</dt>
                                                <dd>
                                                    いいえ、《コードラビリンス　ルーブル》が場に出た時点で、既に《黒幻虫　アラクネ・パイダ》は能力を失っているため、【チャーム】を付けることは出来ません。                                                </dd>
                                                                                            <dt>《不可解な誇超　コンテンポラ》の常時能力によって、《黒幻蟲　アラクネ・パイダ》の常時能力を無効化し、【チャーム】が付いている《非可視の現実　キュビ》の起動能力を使用することはできますか？</dt>
                                                <dd>
                                                    いいえ、《非可視の現実　キュビ》の起動能力を使用することはできません。《不可解な誇超　コンテンポラ》の「効果を受けない」という常時能力は、「カードの状態変化、数値とテキストの変化、効果による移動」を無効化しますが、《黒幻蟲　アラクネ・パイダ》の常時能力は、このどれにも該当しません。                                                </dd>
                                                                                            <dt>対戦相手の《黒幻蟲　アラクネ・パイダ》の下段自動能力が発動したとき、<br />
バニッシュするシグニとして【チャーム】が付いた《先駆の大天使　アークゲイン》を対象とすることはできますか？その場合、バニッシュされますか？</dt>
                                                <dd>
                                                    はい、対象としないこと自体は可能です。しかし、《先駆の大天使　アークゲイン》の「効果を受けない」という常時能力は「カードの状態変化、数値とテキストの変化、効果による移動」を受けませんので、《黒幻蟲　アラクネ・パイダ》の自動能力によってはバニッシュされません。                                                </dd>
                                                                                            <dt>下段自動能力が発動した際に、対戦相手は自身のチャームが付いているシグニを「対象としない」ことはできますか？</dt>
                                                <dd>
                                                    いいえ、対戦相手の効果によってカードを対象とする効果は必ず対象とする必要があります。この場合、「対象としない」ことは出来ません。                                                </dd>
                                                                                            <dt>お互いに【チャーム】がついた《黒幻蟲　アラクネ・パイダ》が1体ずつある状態でアタックフェイズに入った場合、どうなりますか？</dt>
                                                <dd>
                                                    それぞれの《黒幻蟲　アラクネ・パイダ》下段自動能力が、アタックフェイズに入ったときに発動条件を満たしますが、これらはターンプレイヤーから先に発動となります。ターンプレイヤー側の《黒幻蟲　アラクネ・パイダ》の自動能力を先に処理し、次に非ターンプレイヤー側の《黒幻蟲　アラクネ・パイダ》の自動能力を処理します。仮にターンプレイヤー側の処理により、非ターンプレイヤー側の《黒幻蟲　アラクネ・パイダ》がバニッシュされたとしても、一度トリガーした非ターンプレイヤー側の自動能力は発動します。                                                </dd>
                                                                                            <dt>対戦相手の場に《黒幻蟲　アラクネ・パイダ》がいる状態で《聖技の護り手　ラビエル》を出し、出現時能力を《黒幻蟲　アラクネ・パイダ》に使用しました。この《聖技の護り手　ラビエル》に【チャーム】はつきますか？</dt>
                                                <dd>
                                                    《黒幻蟲　アラクネ・パイダ》中段自動能力、《聖技の護り手　ラビエル》出現時能力ともに《聖技の護り手　ラビエル》が場に出たときに発動条件を満たし、ターンプレイヤー側から先に発動し、次に非ターンプレイヤー側が発動します。また、ターンプレイヤー側の発動により《黒幻蟲　アラクネ・パイダ》が先に手札に戻されたとしても、一度トリガーした自動能力は発動しますので、【チャーム】はつくことになります。                                                </dd>
                                                                                            <dt>【シャドウ】を持つシグニに対して出現時能力によって【チャーム】をつけることは可能ですか？</dt>
                                                <dd>
                                                    付けることが可能です。この能力は「シグニ１体が出たことによってトリガーする能力」であり、「シグニ１体を対象としている能力」ではないため、【シャドウ】を持つシグニにチャームを付けることが可能です。                                                </dd>
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
    assert_eq!(card.unwrap().card_type, CardType::Resona);
}
