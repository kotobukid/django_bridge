use webapp::analyze::wixoss::{card::Signi, Card, WixossCard};

use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

use std::sync::Arc;
use std::time::Duration;
use models::gen::django_models::WixCardKlassRel;
use models::card::CreateCard;
use models::klass::create_klass;
use webapp::repositories::{CardRepository, KlassRelRepository};

async fn create_db() -> Pool<Postgres> {
    let workspace_env = format!(
        "{}/.env",
        env::var("CARGO_WORKSPACE_DIR").unwrap_or_default()
    );
    let env_paths = [
        ".env",                 // カレントディレクトリ
        "../.env",              // 一つ上のディレクトリ
        "../../.env",           // 二つ上のディレクトリ（nested crateの場合）
        workspace_env.as_str(), // CARGO_WORKSPACE_DIRが設定されている場合
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
    pool
}

async fn db(
    pool: Arc<Pool<Postgres>>,
    item: CreateCard,
) -> Result<models::card::Card, sqlx::Error> {
    let card_repo = CardRepository::new(pool.clone());
    Ok(card_repo.upsert(item).await?)
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
                                <p class="cardNum">PR-213</p>
                                <p class="cardName">小剣　ミカムネ（WIXOSS PARTY 参加賞 selectors pack vol7）<br class="sp"><span>＜ショウケンミカムネ＞</span></p>
                                <div class="cardRarity">PR</div>
                            </div>
                            <div class="cardImg">
                                                                    <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/PR/PR-213.jpg">
                                                                <p>Illust <span>クロサワテツ</span></p>
                            </div>
                            <div class="cardData">
                                <dl>
                                    <dt>カード種類</dt>
                                    <dd>シグニ</dd>

                                    <dt>カードタイプ</dt>
                                    <dd>精武：アーム/ウェポン</dd>

                                    <dt>色</dt>
                                    <dd>白</dd>

                                    <dt>レベル</dt>
                                    <dd>1</dd>

                                    <dt>グロウコスト</dt>
                                    <dd>-</dd>

                                    <dt>コスト</dt>
                                    <dd>-</dd>

                                    <dt>リミット</dt>
                                    <dd>-</dd>

                                    <dt>パワー</dt>
                                    <dd>2000</dd>

                                    <!-- チーム -->
                                    <dt>限定条件</dt>
                                    <dd>-</dd>
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
										                                        <img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_regular.png" height="23" alt="【常】" />：あなたの場に白ではないシグニがある場合、このシグニの基本パワーは5000になる。<br />
<img src="https://www.takaratomy.co.jp/products/wixoss/img/card/icon/icon_txt_regular.png" height="23" alt="【常】" />：このシグニがエナゾーンにあるかぎり、あなたは自分のセンタールリグが持つ色のエナ１つを支払う際に、代わりにあなたのエナゾーンからこのシグニをトラッシュに置いてもよい。                                    </div>

                                                                    <div class="cardText mb20">
                                        予は天下五剣の１つにして、最も美なる剣であるぞ。～ミカムネ～                                    </div>

                                                                                                    <div class="cardFaq">
                                        <p class="faqTtl">FAQ</p>
                                        <dl>
                                                                                            <dt>
																										《小剣　ミカムネ》、白のシグニ、黒のシグニのように場にある場合、《小剣　ミカムネ》のパワーはいくつになりますか？
												</dt>
                                                <dd>
																										《小剣　ミカムネ》上段常時能力は、白以外のシグニが１体でもいれば条件を満たしパワーが5000となります。同時に白のシグニがいてもそれは影響しません。                                                </dd>
                                                                                            <dt>
																										《小剣　ミカムネ》と《サーバント　Ｏ》のような無色のシグニが場にある場合、《小剣　ミカムネ》のパワーはいくつになりますか？
												</dt>
                                                <dd>
																										《サーバント　Ｏ》がいる場合、それは無色であり白以外のシグニですので、《小剣　ミカムネ》のパワーは5000となります。                                                </dd>
                                                                                            <dt>
																										《黒点の巫女　タマヨリヒメ》などで、色が白以外に変更された《小剣　ミカムネ》だけが場にある場合、《小剣　ミカムネ》のパワーはいくつになりますか？
												</dt>
                                                <dd>
																										《小剣　ミカムネ》自身の色が白以外になった場合、《小剣　ミカムネ》上段常時能力は条件を満たし、パワーは5000となります。                                                </dd>
                                                                                            <dt>
																										自分のルリグが《コード・ピルルク　VERMILION》の場合、エナゾーンの《小剣　ミカムネ》は常時能力でどの色のエナの代わりにすることができますか？
												</dt>
                                                <dd>
																										《コード・ピルルク　VERMILION》は青と黒の色を持つルリグですので、青か黒のエナを支払う際に代わりに《小剣　ミカムネ》をエナゾーンからトラッシュに置けます。<br />
常時能力を使用せず、《小剣　ミカムネ》の本来の色として白エナを支払うこともできます。                                                </dd>
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
    let pool = create_db().await;
    let pool = Arc::new(pool);
    let mut klass_rel_repo: KlassRelRepository = KlassRelRepository::new(pool.clone());
    klass_rel_repo.create_cache().await;

    let signi = Signi::from_source(source);

    // println!("{:?}", signi);

    let klass = create_klass(signi.klass.clone().as_str());
    let klass_found = klass_rel_repo.get_id_by_create_klass(&klass);

    let klass_id = match klass_found {
        Some(id) => id,
        None => klass_rel_repo.create_klass_if_not_exists(klass).await?,
    };

    let card: Card = signi.into();

    let cc: CreateCard = card.into();

    let created_card: models::card::Card = db(pool, cc).await?;

    let rel: WixCardKlassRel = WixCardKlassRel {
        id: -1,
        card_id: created_card.id,
        klass_id,
    };

    klass_rel_repo.save(rel.clone()).await;

    // println!("{:?}", rel);

    Ok(())
}
