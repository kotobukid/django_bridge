use webapp::analyze::wixoss::{card::Arts, WixossCard, Card};

use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

use std::sync::Arc;
use std::time::Duration;
use models::card::CreateCard;
use webapp::repositories::CardRepository;

async fn db(item: CreateCard) -> Result<(), sqlx::Error> {
    let workspace_env = format!("{}/.env", env::var("CARGO_WORKSPACE_DIR").unwrap_or_default());
    let env_paths = [
        ".env",                    // カレントディレクトリ
        "../.env",                 // 一つ上のディレクトリ
        "../../.env",              // 二つ上のディレクトリ（nested crateの場合）
        workspace_env.as_str(),    // CARGO_WORKSPACE_DIRが設定されている場合
    ];

    for path in &env_paths {
        if std::path::Path::new(path).exists() {
            from_filename(path).ok();
            break;
        }
    }

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
    let source: String = r#"<div id="primary" class="content-area">
        <main id="main" class="site-main" role="main">



                <section class="mordal">
                    <div class="cardDetail">
                        <!--<button class="close"><i class="fas fa-times"></i></button>-->
                        <div class="cardDetailWrap">
                            <div class="cardttlwrap">
                                <p class="cardNum">WX24-P4-005</p>
                                <p class="cardName">竜花相搏<br class="sp"><span>＜リュウカソウハク＞</span></p>
                                <div class="cardRarity">LR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WX24/WX24-P4-005.jpg">
                                                                <p>Illust <span>夕子</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>アーツ</dd>

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
《緑》×１</dd>

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
										                                        あなたのデッキをシャッフルし一番上のカードをライフクロスに加える。あなたのライフクロス１枚をクラッシュしてもよい。そうした場合、対戦相手のライフクロス１枚をクラッシュする。                                    </div>

                                                                    <div class="cardText mb20">
                                        今なら、きっと過去の自分たちにも勝つことができる――<br />
自然と、自信がわいてくるのだった。                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>
																										この効果でクラッシュする自分のライフクロスは、直前の効果で加えたカードですか？
												</dt>
                                                <dd>
																										はい、最初の効果でライフクロスの1番上に1枚加え、それをクラッシュすることになります。ライフクロスを加えたりクラッシュするのは常に一番上からに対してとなります。                                                </dd>
                                                                                            <dt>
																										この効果で自分と相手のライフクロスをそれぞれクラッシュしたら、お互いのカードにライフバーストがありました。どういう順番で発動しますか？
												</dt>
                                                <dd>
																										ライフバーストはトリガー能力であり、トリガーしている能力はターンプレイヤー側から先に発動します。メインフェイズにこのアーツを使用した場合、あなたがターンプレイヤーですので、あなたのライフバーストが先に発動します。                                                </dd>
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
    // println!("{}", card);
    let cc: CreateCard = card.into();

    db(cc).await?;

    Ok(())
}
