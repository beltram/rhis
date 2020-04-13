echo "Building WASM binary"
cd ..
cargo build --release --target=wasm32-unknown-unknown
echo "Building OCI image"
wasme build precompiled -t webassemblyhub.io/$WASME_USER/rhis:$1 ./target/wasm32-unknown-unknown/release/rhis.wasm
echo "Pushing image to webassemblyhub.io"
wasme push webassemblyhub.io/$WASME_USER/rhis:$1
echo "Clearing existing filters"
wasme undeploy istio --id=rhis --namespace rhis
sleep 20
echo "Deploying image to namespace 'rhis'"
wasme deploy istio webassemblyhub.io/$WASME_USER/rhis:$1 --id=rhis --namespace rhis