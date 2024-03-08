from django.db import models


class LowercaseCharField(models.CharField):
    def to_python(self, value):
        return value.lower()


class User(models.Model):
    username = LowercaseCharField(max_length=20, unique=True)
    password = models.CharField(max_length=20)  # TODO: Fix this security issue
