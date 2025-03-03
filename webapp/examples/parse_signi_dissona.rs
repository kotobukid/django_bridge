use webapp::analyze::wixoss::{Card, card::Signi, WixossCard};

use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

use std::sync::Arc;
use std::time::Duration;
use webapp::models::card::CreateCard;
use webapp::repositories::CardRepository;

async fn db(item: CreateCard) -> Result<(), sqlx::Error> {
    from_filename("../.env").ok();

    let db_url = {
        let host = env::var("DB_HOST").expect("DB_HOST not found in .env");
        let port = env::var("DB_PORT").expect("DB_PORT not found in .env");
        let user = env::var("DB_USER").expect("DB_USER not found in .env");
        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD not found in .env");
        let db_name = env::var("DB_NAME").expect("DB_NAME not found in .env");
        format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, db_name
        )
    };

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(format!("{db_url}?connect_timeout=5").as_str())
        .await
        .expect("Failed to connect to database");

    let pool = Arc::new(pool);

    let card_repo = CardRepository::new(pool.clone());
    card_repo.insert(item).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let source: String = r#"
    <div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WXDi-P12-071</p>
                                <p class="cardName">コードイート　マチャフラ//ディソナ<br class="sp"><span>＜コードイートマチャフラディソナ＞</span></p>
                                <div class="cardRarity">C</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXDi/WXDi-P12-071.jpg">
                                                                <p>Illust <span>志月</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>シグニ</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>奏械：調理</dd>

                                    <dt>色</dt>
                                    <dd>青</dd>

                                    <dt>レベル</dt>
                                    <dd>1</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>-</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>3000</dd>

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
                                                                                <picture class=cardData_story_img_wrapper>
                                            <source srcset="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_story_dissona.png" media="(min-width: 768px)" />
                                            <img class="cardData_story_img" src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_dissona.png" alt="" />
                                        </picture>
                                                                        </dd>
                                </dl>

                                                                    <div class="cardSkill">
                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_auto.png" height="23" alt="【自】" /><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_turn_01.png" height="23" alt="《ターン１回》" />：あなたが<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_dissona.png" height="23" alt="《ディソナアイコン》" />のカードを１枚捨てたとき、対戦相手のシグニ１体を対象とし、それを凍結する。<br />
（凍結されたシグニは次の自分のアップフェイズにアップしない）<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_arrival.png" height="23" alt="【出】" />手札から<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_dissona.png" height="23" alt="《ディソナアイコン》" />のカードを１枚捨てる：カードを１枚引く。                                    </div>
                                                                                                    <div class="cardSkill">
                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_burst.png" width="26" height="24" alt="ライフバースト" />：対戦相手のシグニ１体を対象とし、それをダウンし凍結する。カードを１枚引く。                                    </div>

                                                                    <div class="cardText mb20">
                                        「さぁ、お召し上がれ！」                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>自分の場に、出現時能力を発動した《蒼美　ふたせ//ディソナ》と《コードイート　マチャフラ//ディソナ》があります。《蒼美　ふたせ//ディソナ》がアタックし、自動能力を発動して《ディソナアイコン》のカードを捨てました。それにより《コードイート　マチャフラ//ディソナ》の自動能力で、《蒼美　ふたせ//ディソナ》の正面のシグニを凍結した場合、それがパワー5000以下なら《蒼美　ふたせ//ディソナ》は【アサシン】によって対戦相手にダメージを与えられますか？</dt>
                                                <dd>
                                                    はい、【アサシン】が有効になり、バトルをせずに対戦相手にダメージを与えます。常時能力は条件を満たしたら即時に効果を適用します。アタックしたときの自動能力やそれによってトリガーする能力をすべて処理した後にバトルやダメージの処理に入りますので、その時点で【アサシン】を持っていれば対戦相手にダメージを与えることができます。                                                </dd>
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

    let cc: CreateCard = card.into();

    db(cc).await?;

    Ok(())
}
