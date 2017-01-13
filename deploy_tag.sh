#!/bin/bash
set -e; 
cd target/release

releases_url="https://api.github.com/repos/${CIRCLE_PROJECT_USERNAME}/${CIRCLE_PROJECT_REPONAME}/releases/tags/${CIRCLE_TAG}";
release_info=`curl -H "Authorization: token ${GITHUB_TOKEN}" $releases_url`
upload_url=`echo $release_info | jq -r ".upload_url"`;
upload_url="${upload_url/\{?name,label\}/?name=linux.tar.gz}"

tar -cvzf release.tar.gz yggdrasil2;

curl --data-binary @release.tar.gz -H "Content-Type: application/gzip" -H "Authorization: token ${GITHUB_TOKEN}" $upload_url;