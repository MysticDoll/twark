#/bin/bash

echo access "https://id.twitch.tv/oauth2/authorize?client_id=${TWARK_CLIENT_ID}&redirect_uri=http://localhost:8000/twark/&response_type=code&scope=chat:read+chat:write"

CODE=$(nc -lp8000 | head -1 | cut -d'?' -f2 | cut -d'=' -f2 | cut -d'&' -f1)
export TWARK_ACCESS_TOKEN=$(curl -XPOST "https://id.twitch.tv/oauth2/token?client_id=${TWARK_CLIENT_ID}&client_secret=${TWARK_CLIENT_SECRET}&code=${CODE}&grant_type=authorization_code&redirect_uri=http://localhost:8000/twark/" | jq -r '.access_token')
