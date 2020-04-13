#kubectl run curl --image=radial/busyboxplus:curl -i --tty
#curl http://rhis:80/get

kubectl exec -ti -n bookinfo deploy/productpage-v1 -c istio-proxy -- curl -v http://details.bookinfo:9080/details/123