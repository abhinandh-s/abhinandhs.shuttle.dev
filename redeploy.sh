#/usr/bin/sh
shuttle_redeploy() {
  shuttle project delete --name "$1" && shuttle deploy --name "$1"
}
