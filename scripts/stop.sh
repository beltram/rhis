echo "Remove all filters"
wasme undeploy istio --id rhis --namespace bookinfo
echo "Remove all pods"
kubectl delete -n bookinfo -f ./bookinfo/bookinfo.yaml
echo "Remove Istio"
sh -t ./bookinfo/cleanup.sh
echo "Kill cluster"
k3d stop
