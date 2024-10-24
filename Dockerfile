FROM postgres:15-bookworm AS database
# 必要なパッケージをインストールし、日本語ロケールを生成
RUN apt-get update && \
    apt-get install -y locales && \
    echo "ja_JP.UTF-8 UTF-8" > /etc/locale.gen && \
    locale-gen ja_JP.UTF-8 && \
    update-locale LANG=ja_JP.UTF-8 LC_ALL=ja_JP.UTF-8 && \
    locale -a && \
    rm -rf /var/lib/apt/lists/*

# 環境変数を設定して日本語ロケールをデフォルトに
ENV LANG=ja_JP.UTF-8
ENV LC_ALL=ja_JP.UTF-8