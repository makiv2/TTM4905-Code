import json
import os

from .base import *

# TODO REMOVE ALLOWED HOSTS 0.0.0.0
ALLOWED_HOSTS = [
    "127.0.0.1",
    "django-backend"
]

# SECURITY WARNING: don't run with debug turned on in production!
DEBUG = False

# CSRF token TODO

SECURE_PROXY_SSL_HEADER = ("HTTP_X_FORWARDED_PROTO", "https")


DATABASES = {
    "default": {
        "ENGINE": "django.db.backends.postgresql",
        "NAME": os.environ.get("DB_NAME"),
        "USER": os.environ.get("DB_USERNAME"),
        "PASSWORD": os.environ.get("DB_PASSWORD"),
        "HOST": os.environ.get("DB_HOST"),
        "PORT": os.environ.get("DB_PORT"),
    }
}

# SECURITY WARNING: keep the secret key used in production secret!
SECRET_KEY = os.environ.get("SECRET_KEY")
