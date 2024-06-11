import json
import os
from pathlib import Path

from django.conf import settings
from django.core.management.base import BaseCommand

from pubkeys.models import Pubkey


class Command(BaseCommand):
    help = "Create public keys from JSON file"

    def handle(self, *args, **kwargs):
        # set the path to the datafile
        datafile = Path(os.path.dirname(settings.BASE_DIR)) / "data" / "pubkeys.json"
        print("----------------------------------------------------")
        print("----------------------------------------------------")
        print(datafile)
        print("----------------------------------------------------")
        print("----------------------------------------------------")
        assert datafile.exists()

        # load the datafile
        with open(datafile, "r") as f:
            data = json.load(f)

        # convert list of dictionaries to list of Track models, and bulk_create
        pubkeys = [Pubkey(**pubkey) for pubkey in data]

        Pubkey.objects.bulk_create(pubkeys)
