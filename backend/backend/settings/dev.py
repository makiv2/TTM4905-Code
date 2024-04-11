from os import environ as env

from .base import *

try:
    from .local import *
except ImportError:
    pass

ALLOWED_HOSTS = ["localhost", "127.0.0.1", "django-backend"]


# SECURITY WARNING: don't run with debug turned on in production!
DEBUG = True

# SECURITY WARNING: keep the secret key used in production secret!
SECRET_KEY = "django-insecure-pned@!u8qc4vh2)f+_oveg1k$%h6t8kq2j37y5=w*$#jfd=sd%"

# Database
# https://docs.djangoproject.com/en/3.2/ref/settings/#databases


DATABASES = {
    "default": {
        "ENGINE": "django.db.backends.postgresql",
        "NAME": env.get("DB_NAME"),
        "USER": env.get("DB_USER"),
        "PASSWORD": env.get("DB_PASSWORD"),
        "HOST": env.get("DB_HOST"),
        "PORT": "5432",
    }
}
