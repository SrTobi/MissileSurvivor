#!/usr/bin/env bash

set -e

HELP_STRING=$(cat <<- END
	usage: build_wasm.sh
	Build script for combining a Macroquad project with wasm-bindgen,
	allowing integration with the greater wasm-ecosystem.
	example: ./build_wasm.sh
	This'll go through the following steps:
	    1. Build as target 'wasm32-unknown-unknown'.
	    2. Create the directory 'dist' if it doesn't already exist.
	    3. Run wasm-bindgen with output into the 'dist' directory.
	    4. Apply patches to the output js file (detailed here: https://github.com/not-fl3/macroquad/issues/212#issuecomment-835276147).
        5. Generate coresponding 'index.html' file.
	Author: Tom Solberg <me@sbg.dev>
	Edit: Nik codes <nik.code.things@gmail.com>
	Edit: Nobbele <realnobbele@gmail.com>
	Version: 0.2
END
)


die () {
    echo >&2 "Error: $@"
    echo >&2
    echo >&2 "$HELP_STRING"
    exit 1
}

BUILD_TYPE=debug
BUILD_FLAGS=""

# Parse primary commands
while [[ $# -gt 0 ]]
do
    key="$1"
    case $key in
        --release)
            RELEASE=yes
            BUILD_TYPE=release
            BUILD_FLAGS="--release"
            shift
            ;;

        -h|--help)
            echo "$HELP_STRING"
            exit 0
            ;;

        *)
            POSITIONAL+=("$1")
            shift
            ;;
    esac
done

# Restore positionals
set -- "${POSITIONAL[@]}"

PROJECT_NAME="MCom"

HTML=$(cat <<- END
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>EconTest</title>
    <base href="." />
    <style>
        html,
        body,
        canvas {
            margin: 0px;
            padding: 0px;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            z-index: 0;
        }
    </style>
</head>
<body style="margin: 0; padding: 0; height: 100vh; width: 100vw;">
    <canvas id="glcanvas" tabindex='1'></canvas>
    <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
    <script type="module">
        import { set_wasm, get_imports } from "./${PROJECT_NAME}.js";
        async function impl_run() {
            miniquad_add_plugin({
                register_plugin: (a) => (a.wbg = get_imports()),
                on_init: () => set_wasm(wasm_exports),
                version: "0.0.1",
                name: "wbg",
            });
            load("./${PROJECT_NAME}_bg.wasm");
        }
        window.run = function() {
            //document.getElementById("run-container").remove();
            //document.getElementById("glcanvas").removeAttribute("hidden");
            document.getElementById("glcanvas").focus();
            impl_run();
        }
        run()
    </script>
</body>
</html>
END
)

# Build
export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
cargo build --bin MCom $BUILD_FLAGS --target wasm32-unknown-unknown



# Generate bindgen outputs
mkdir -p dist/$BUILD_TYPE
wasm-bindgen target/wasm32-unknown-unknown/$BUILD_TYPE/$PROJECT_NAME.wasm --out-dir dist/$BUILD_TYPE --target web --no-typescript

# Shim to tie the thing together
sed -i "s/import \* as __wbg_star0 from 'env';//" dist/$BUILD_TYPE/$PROJECT_NAME.js
sed -i "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" dist/$BUILD_TYPE/$PROJECT_NAME.js
sed -i "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" dist/$BUILD_TYPE/$PROJECT_NAME.js
#sed -i "s/const imports = getImports();/return getImports();/" dist/$BUILD_TYPE/$PROJECT_NAME.js
sed -i -e '$aexport const get_imports = __wbg_get_imports;' dist/$BUILD_TYPE/$PROJECT_NAME.js
sed -i "s/return imports;//" dist/$BUILD_TYPE/$PROJECT_NAME.js

# Create index from the HTML variable
echo "$HTML" > dist/$BUILD_TYPE/index.html
