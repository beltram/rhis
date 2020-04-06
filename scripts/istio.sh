curl -L https://istio.io/downloadIstio | ISTIO_VERSION=1.5.1 sh
cd istio-1.5.1
bin/istioctl manifest apply --set profile=demo