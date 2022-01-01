mv trunk-config-release.toml Trunk.toml
trunk build --release
mv Trunk.toml trunk-config-release.toml
cp build/* /home/dys/dev/www/dystroy/dysp/
/home/dys/dev/www/dystroy/deploy.sh
