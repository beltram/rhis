echo "Starting cluster"
k3d start
echo "Waiting for cluster deployment..."
sleep 30
echo "Deploying Istio"
istioctl manifest apply --set profile=demo
echo "Applying WASME CRDs"
kubectl apply -f https://github.com/solo-io/wasme/releases/latest/download/wasme.io_v1_crds.yaml
kubectl apply -f https://github.com/solo-io/wasme/releases/latest/download/wasme-default.yaml
echo "Starting bookinfo"
kubectl create ns bookinfo
kubectl label namespace bookinfo istio-injection=enabled --overwrite
kubectl apply -n bookinfo -f ./bookinfo/bookinfo.yaml
echo "Waiting for bookinfo deployment..."
sleep 40
kubectl exec $(kubectl get pod --selector app=details --output jsonpath='{.items[0].metadata.name}') -c istio-proxy -- curl -X POST http://localhost:15000/logging?level=info
echo "Deploying rhis filter"
sh build.sh $1