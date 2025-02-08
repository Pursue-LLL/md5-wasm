# md5-rs

A WebAssembly module for MD5 implementation in Rust

## Usage

```javascript
npm install @gogors/md5-wasm
```

### Node.js
```javascript
const { md5 } = require('@gogors/md5-wasm');

console.log(md5('hello')); // output: 5d41402abc4b2a76b9719d911017c592
```

### Browser Usage

ES Module

```html
<script type="module">
  import init, { md5 } from 'https://unpkg.com/@gogors/md5-wasm/pkg/web/md5_wasm.js';
  init().then(() => {
    console.log(md5('hello')); // output: 5d41402abc4b2a76b9719d911017c592
  });
</script>
```

Non-Module

```html
<script src="https://unpkg.com/@gogors/md5-wasm/pkg/no-modules/md5_wasm.js"></script>
<script>
  (async () => {
      // Initialize wasm
      await wasm_bindgen();

      // Get md5 function
      const { md5 } = wasm_bindgen;
    console.log(md5('hello')); // output: 5d41402abc4b2a76b9719d911017c592
  })();
</script>
```

### Usage with Build Tools

For Vite, you need to use the vite-plugin-wasm plugin

> Reference: https://github.com/Menci/vite-plugin-wasm

```bash
npm install vite-plugin-wasm
```

```typescript
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait()
  ]
});
```

For Webpack, you need to use the @wasm-tool/wasm-pack-plugin plugin

```bash
npm install @wasm-tool/wasm-pack-plugin
```

```typescript
import wasmPackPlugin from '@wasm-tool/wasm-pack-plugin';

export default defineConfig({
  plugins: [wasmPackPlugin()],
});
```

For Rollup, you need to use the rollup-plugin-esmwasm plugin

> Reference: https://github.com/Pursue-LLL/rollup-plugin-esmwasm

```bash
npm install rollup-plugin-wasm
```

```typescript
import wasm from 'rollup-plugin-esmwasm';

export default {
  plugins: [wasm()],
};
```