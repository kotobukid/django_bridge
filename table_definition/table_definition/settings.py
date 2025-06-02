"""
Django Bridge - 管理画面専用Django設定

このプロジェクトは以下の特殊な用途で使用されます：
1. Axumプロジェクト（shared/webapp）からの一時的なDjango開発サーバー起動
2. Djangoのモデル定義とマイグレーション機能の利用
3. Django管理画面の自動生成機能の利用

通常のWebアプリケーションとは異なり、管理画面とモデル管理のみに特化しています。
"""
import os
from pathlib import Path
import environ
from .config import (
    TRUSTED_ORIGINS_BASE, ADMIN_ONLY_APPS, PROJECT_APPS, 
    DEVELOPMENT_SECURITY, LOCALE_SETTINGS
)

env = environ.Env()

# プロジェクトのベースディレクトリ
BASE_DIR = Path(__file__).resolve().parent.parent

# 環境変数の読み込み
environ.Env.read_env(os.path.join(BASE_DIR, '../.env'))

# セキュリティ設定
SECRET_KEY = env("SECRET_KEY")
DEBUG = DEVELOPMENT_SECURITY['DEBUG']
ALLOWED_HOSTS = DEVELOPMENT_SECURITY['ALLOWED_HOSTS']

# CSRF設定（Axumサーバーとの連携用）
CSRF_TRUSTED_ORIGINS = TRUSTED_ORIGINS_BASE.copy()
# アプリケーション定義（管理画面専用構成）
INSTALLED_APPS = ADMIN_ONLY_APPS + PROJECT_APPS

MIDDLEWARE = [
    'django.middleware.security.SecurityMiddleware',
    'django.contrib.sessions.middleware.SessionMiddleware',
    'django.middleware.common.CommonMiddleware',
    'django.middleware.csrf.CsrfViewMiddleware',
    'django.contrib.auth.middleware.AuthenticationMiddleware',
    'django.contrib.messages.middleware.MessageMiddleware',
    'django.middleware.clickjacking.XFrameOptionsMiddleware',
]

ROOT_URLCONF = 'table_definition.urls'

TEMPLATES = [
    {
        'BACKEND': 'django.template.backends.django.DjangoTemplates',
        'DIRS': [],
        'APP_DIRS': True,
        'OPTIONS': {
            'context_processors': [
                'django.template.context_processors.debug',
                'django.template.context_processors.request',
                'django.contrib.auth.context_processors.auth',
                'django.contrib.messages.context_processors.messages',
            ],
        },
    },
]

WSGI_APPLICATION = 'table_definition.wsgi.application'

# データベース設定（Rustプロジェクトと共有）
DATABASES = {
    'default': {
        'ENGINE': 'django.db.backends.postgresql',
        'NAME': env("DB_NAME"),
        'USER': env("DB_USER"),
        'PASSWORD': env("DB_PASSWORD"),
        'HOST': env("DB_HOST"),
        'PORT': env("DB_PORT"),
    },
}

# Password validation
# https://docs.djangoproject.com/en/5.1/ref/settings/#auth-password-validators

AUTH_PASSWORD_VALIDATORS = [
    {
        'NAME': 'django.contrib.auth.password_validation.UserAttributeSimilarityValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.MinimumLengthValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.CommonPasswordValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.NumericPasswordValidator',
    },
]

# 国際化設定
LANGUAGE_CODE = LOCALE_SETTINGS['LANGUAGE_CODE']
TIME_ZONE = LOCALE_SETTINGS['TIME_ZONE']
USE_I18N = LOCALE_SETTINGS['USE_I18N']
USE_TZ = LOCALE_SETTINGS['USE_TZ']

# 静的ファイル設定（管理画面用）
# CUSTOM_ADMIN_ROOTは run_with_custom_admin コマンドで動的に設定される
if globals().get("CUSTOM_ADMIN_ROOT") is None:
    CUSTOM_ADMIN_ROOT = ""

STATIC_URL = f"{CUSTOM_ADMIN_ROOT}a_static/"
STATIC_ROOT = os.path.join(BASE_DIR, 'static')

# Django設定
DEFAULT_AUTO_FIELD = 'django.db.models.BigAutoField'

# ログ設定（開発サーバーの警告を非表示にする）
LOGGING = {
    'version': 1,
    'disable_existing_loggers': False,
    'handlers': {
        'console': {
            'class': 'logging.StreamHandler',
            'level': 'ERROR',  # WARNING以下を非表示
        },
    },
    'loggers': {
        'django.server': {
            'handlers': ['console'],
            'level': 'ERROR',
            'propagate': False,
        },
    },
}
