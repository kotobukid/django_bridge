use webapp::analyze::wixoss::{card::Arts, Card, WixossCard};

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
                                <p class="cardNum">WX03-003</p>
                                <p class="cardName">業火絢爛<br class="sp"><span>＜ゴウカケンラン＞</span></p>
                                <div class="cardRarity">LR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/WX03/WX03-003.jpg">
                                                                <p>Illust <span>コウサク</span></p>
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
《無》×３</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>-</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>-</dd>
                                    <!-- コイン -->
                                    <dt>使用タイミング</dt>
                                    <dd>メインフェイズ<br />
アタックフェイズ</dd>

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
										                                        対戦相手のセンタールリグが緑の場合、このカードの基本コストは<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_red_02.png" height="23" alt="《赤×2》" />になる。<br />
<br />
対戦相手のパワー15000以下のシグニ１体を対象とし、それをバニッシュする。                                    </div>

                                                                    <div class="cardText mb20">
                                        燃えやすい色ね！                                    </div>

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
