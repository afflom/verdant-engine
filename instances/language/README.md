# Verdant Language

A browser-local Verdant instance for free-form text. The game supplies the current player objective and last dialogue from its Wasm story interface on each turn. An optional text observation can supply other content. The afflom/me SOSA guest checks the observation graph. SmolLM2-135M-Instruct generates the response using the imported tokenizer, Transformers.js and ONNX Runtime Web. Generation takes place in the verified me worker. The model does not modify game state.

From this directory, run `python3 prepare.py` and `bash build.sh` (Node 22+, Git and Docker). Then run `python3 -m http.server 8751 --bind 127.0.0.1` and open http://localhost:8751. The downloadable verified instance includes the prepared runtime and model objects and only needs the local web server.

The generated Prism applies the upstream model-admission rules and invariants. The browser executes its verified holo on fetch and inference requests. Build inputs are pinned in build.sh and prepare.py. The full model and tokenizer digests are in the imported models.lock.json. The generator timestamp is fixed for reproducibility. To replay the browser test, install the engine's locked npm dependencies and Playwright Chromium, start the local server, and run `node verify.mjs` here.

The checked text observation records its source and player perspective. The transcript retains that observation, the engine snapshot, the model revision and the generated words. A schema check establishes the structure of an observation. The factual accuracy of a model response is evaluated separately against its input.
