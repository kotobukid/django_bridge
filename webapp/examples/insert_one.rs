use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

use std::sync::Arc;
use std::time::Duration;
use webapp::analyze::wixoss::{card::Arts, Card, WixossCard};
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
    card_repo.upsert(item).await?;

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
                                <p class="cardNum">WXK11-002</p>
                                <p class="cardName">四炎楚歌<br class="sp"><span>＜シエンソカ＞</span></p>
                                <div class="cardRarity">LR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WXK11/WXK11-002.jpg">
                                                                <p>Illust <span>しおぼい</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>アーツ</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>-</dd>

                                    <dt>色</dt>
                                    <dd>赤</dd>

                                    <dt>レベル</dt>
                                    <dd>-</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>《赤》×２<br />
《無》×２</dd>

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
                                    <dd><img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_format_key.png" height="23" alt="《キーアイコン》" /></dd>

                                    <!-- 0205mao -->
                                    <!-- 0205niimura -->
                                    <dt>ストーリー</dt>
                                    <dd>
                                                                            -
                                                                        </dd>
                                </dl>

                                                                    <div class="cardSkill">
                                        このアーツはあなたのセンタールリグがレベル４以上の場合にしか使用できない。 <br />
<br />
以下の４つを行う。<br />
①対戦相手のシグニ１体を対象とし、それをトラッシュに置く。<br />
②対戦相手のライフクロス１枚をトラッシュに置く。<br />
③対戦相手のエナゾーンからカード１枚を対象とし、それをトラッシュに置く。<br />
④対戦相手のセンタールリグの下にあるカード１枚を対象とし、それをルリグトラッシュに置く。                                    </div>

                                                                    <div class="cardText mb20">
                                        もう一度味わってもらうよ！～遊月～                                    </div>

                                                                    <div class="cardFaq mb20">
                                        <p class="faqTtl">四炎楚歌に関するお知らせ一覧</p>
                                        <dl class="limitedInfo">
                                                                                            <dt>2021-04-30</dt>
                                                <dd>
                                                    <p>こちらのカードは【同時使用制限カード】として指定されました。詳細な内容はルールページ<a href="https://www.takaratomy.co.jp/products/wixoss/library/rule/210430">『《ホーリー・グランドスラム》と《ビカム・ユー》《四炎楚歌》の同時使用制限』</a>をご覧ください。</p>
                                                </dd>
                                                                                    </dl>
                                    </div>
                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>このアーツは、対戦相手のシグニやライフクロスがない場合でも使用できますか？</dt>
                                                <dd>
                                                    はい、対象とすることができるカードがない場合でも使用できます。その場合は対象とすることができない部分の効果を無視し、他の効果を処理します。                                                </dd>
                                                                                            <dt>「このシグニが場を離れたとき」にトリガーする自動能力を持ったシグニを、①の効果でトラッシュに置きました。自動能力と、このアーツの②～④はどちらが先に処理されますか？</dt>
                                                <dd>
                                                    トリガー能力は、効果の処理中には発動しません。このアーツの効果は①～④まで１つの効果ですのでまずこのアーツを最後まで処理します。その後に、トリガーしていた能力を発動します。                                                </dd>
                                                                                            <dt>①から④のうち、行いたくないものを飛ばすことはできますか？</dt>
                                                <dd>
                                                    いいえ、できません。例えば①の場合、対象とすることができる対戦相手のシグニがあるなら必ずそのうちの１体を対象とし、トラッシュに置きます。                                                </dd>
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

    let arts = Arts::from_source(source);
    println!("{}", &arts);
    let card: Card = arts.into();

    let cc: CreateCard = card.into();

    db(cc).await?;

    Ok(())
}
