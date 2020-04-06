<img src="https://img.shields.io/badge/version-0.1.0-brightgreen.svg" />

# Rhis

An Envoy filter tracing HTTP request/response elements.

## Build it

* Install [wasme](https://docs.solo.io/web-assembly-hub/latest/tutorial_code/getting_started/)
* Login to [web-assembly-hub](https://webassemblyhub.io/) `wasme login -u $WASME_USER -p $WASME_PWD`
* Build & deploy `cd scripts && sh build.sh v0.1.0`

## Install it
* Install [wasme](https://docs.solo.io/web-assembly-hub/latest/tutorial_code/getting_started/)
* Pull OCI image `wasme pull webassemblyhub.io/beltram/rhis:v0.1.0`
* Deploy image `wasme deploy istio webassemblyhub.io/beltram/rhis:v0.1.0 --id=rhis`
* (Optional) set Envoy log level to info by replacing:
  * `<host>` by your cluster host
  * `<selector>` by Pod's you want to inspect
  * into `kubectl exec $(kubectl get pod --selector app=<selector> --output jsonpath='{.items[0].metadata.name}') -c istio-proxy -- curl -X POST http://<host>:15000/logging?level=info`
    
## Demo
* `cd scripts`
* Bootstrap cluster `sh start.sh`
* curl details service `sh ping.sh`
* `kubectl logs details-<..> -c istio-proxy`
* Prune `sh stop.sh`

## Docs

* [SDK doc](https://github.com/proxy-wasm/proxy-wasm-cpp-sdk/blob/master/docs/wasm_filter.md)