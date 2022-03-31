#!/bin/sh

. /etc/os-release

cat << EOF
{
  "os": {
    "id": "$ID",
    "version": "$VERSION_ID",
    "like": "$ID $ID_LIKE",
    "codename": "$VERSION_CODENAME"
  }
}
EOF

# . /etc/os-release
# echo {\"os\": {\"id\": \"$ID\",\"version\": \"$VERSION_ID\",\"like\": \"$(echo $ID $ID_LIKE)\",\"codename\": \"$VERSION_CODENAME\"}}'