from django.db import models


class Pubkey(models.Model):
    key = models.CharField(max_length=1024, unique=True)

    def __str__(self):
        return self.key
