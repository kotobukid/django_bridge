---

### **1. PostgreSQLのインストール**
まず、最新版のPostgreSQLと必要な開発ツールをインストールします。

```shell
sudo apt update && sudo apt upgrade -y
sudo apt install -y postgresql-server-dev-all postgresql
```

- ファイアウォールを無効化する場合:
  ```shell
  sudo ufw disable
  ```

---

### **2. PostgreSQLの初期設定**

1. **PostgreSQLにログイン（peer認証）**:
   ```shell
   sudo -u postgres psql
   ```

2. **ログイン用ユーザーとデータベースの作成**:
   ```postgresql
   ALTER ROLE <username> WITH PASSWORD '<password>';
   CREATE DATABASE <dbname>;
   \q
   ```

3. **接続テスト（Vagrant内）**:
   ```shell
   psql -h localhost -U <username> -W --dbname <dbname>
   # \q
   ```

---

### **3. 外部接続用の設定**

1. **認証設定ファイルを編集**:  
   設定ファイル `/etc/postgresql/14/main/pg_hba.conf` を編集します。
   ```shell
   sudo vi /etc/postgresql/14/main/pg_hba.conf
   ```

   以下を追加:
   ```
   host   all             postgres        0.0.0.0/0                        md5
   ```

2. **PostgreSQLの接続範囲を広げる**:  
   `/etc/postgresql/14/main/postgresql.conf` を編集します。
   ```shell
   sudo vi /etc/postgresql/14/main/postgresql.conf
   ```

   以下を設定:
   ```
   listen_addresses = '*'
   ```

3. **設定ファイルを編集した場合、PostgreSQLを再起動する**:
   ```shell
   sudo systemctl restart postgresql
   ```

---

### **4. pgvectorのインストール**

pgvectorを利用するには、PostgreSQLの拡張機能として明示的にビルド・インストールする必要があります。

1. **リポジトリをCloneしてセットアップ**:
   ```shell
   sudo sed -i 's/archive.ubuntu.com/jp.archive.ubuntu.com/g' /etc/apt/sources.list
   sudo apt install -y postgresql-server-dev-14 gcc make
   git clone --branch v0.8.0 https://github.com/pgvector/pgvector.git
   cd pgvector
   make
   sudo make install
   ```

2. **PostgreSQLでpgvectorを有効化**:
   PostgreSQLの任意のデータベースで以下のコマンドを実行し、pgvectorを有効にします:
   ```postgresql
   CREATE EXTENSION vector;
   ```

---

### **5. 外部からの接続確認**

クライアント側で以下のコマンドを使用して、pgvectorが動いていることを確認します。環境やネットワーク設定に従ってIPアドレス/ホスト名などを修正してください。

```shell
psql -h <vagrant-ip-address> -U <username> -W --dbname <dbname>
```

---

### **注意事項**

- **pgvectorのバージョン**: 現在 `v0.8.0` をインストールしていますが、最新バージョンが存在する場合、必要に応じて変更してください。
- **セキュリティ**: `pg_hba.conf` および `listen_addresses` で全開放（0.0.0.0）を行っていますが、本番環境では適切な範囲に限定するよう設定してください。
- **依存ツール**: GCCやMakeがインストールされていない場合、明示的にインストールする必要があります。

---

以上の手順で、Vagrant内にPostgreSQLとpgvectorをセットアップできます。必要に応じて設定をカスタマイズしてください！
