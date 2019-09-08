#!/bin/bash

curl "https://api.telegram.org/bot$BOTTOKEN/setWebhook?url=$1/webhook"