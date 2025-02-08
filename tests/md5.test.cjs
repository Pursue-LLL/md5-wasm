// 仅用于测试nodejs版本，无法测试web和bundler版本

/* eslint-disable @typescript-eslint/no-require-imports */
const test = require('ava');
const npmMd5 = require('md5');
const { md5 } = require('../pkg/node/md5_wasm.js');

test('md5 should match npm md5 for empty string', (t) => {
  const result = md5('');
  const expected = npmMd5('');
  t.is(result, expected);
});

test('md5 should match npm md5 for basic string', (t) => {
  const result = md5('hello');
  const expected = npmMd5('hello');
  t.is(result, expected);
});

test('md5 should match npm md5 for unicode strings', (t) => {
  const str = '你好，世界';
  const result = md5(str);
  const expected = npmMd5(str);
  t.is(result, expected);
});

test('md5 should match npm md5 for special characters', (t) => {
  const str = '!@#$%^&*()_+-=[]{}|;:,.<>?';
  const result = md5(str);
  const expected = npmMd5(str);
  t.is(result, expected);
});

// 添加性能测试
function generateRandomString(length) {
  return 'x'.repeat(length);
}

function measureTime(fn) {
  const start = process.hrtime.bigint();
  fn();
  const end = process.hrtime.bigint();
  return Number(end - start) / 1_000_000; // 转换为毫秒
}

function runBenchmark(input, iterations = 100) {
  // 预热次数
  for (let i = 0; i < 10; i++) {
    md5(input);
    npmMd5(input);
  }

  // 测试 WASM 实现
  const wasmTime = measureTime(() => {
    for (let i = 0; i < iterations; i++) {
      md5(input);
    }
  });

  // 测试 npm md5 实现
  const npmTime = measureTime(() => {
    for (let i = 0; i < iterations; i++) {
      npmMd5(input);
    }
  });

  return {
    wasmTime: wasmTime / iterations,
    npmTime: npmTime / iterations,
    improvement: `${(npmTime / wasmTime).toFixed(2)}x`,
  };
}

test('Performance comparison - small input (32 bytes)', (t) => {
  const input = generateRandomString(32);
  const result = runBenchmark(input, 100);

  console.log('\nSmall input (32 bytes) performance:');
  console.log(`WASM MD5: ${result.wasmTime.toFixed(3)}ms`);
  console.log(`NPM MD5:  ${result.npmTime.toFixed(3)}ms`);
  console.log(`WASM is ${result.improvement} faster`);

  t.pass();
});

test('Performance comparison - medium input (1KB)', (t) => {
  const input = generateRandomString(1024);
  const result = runBenchmark(input, 50);

  console.log('\nMedium input (1KB) performance:');
  console.log(`WASM MD5: ${result.wasmTime.toFixed(3)}ms`);
  console.log(`NPM MD5:  ${result.npmTime.toFixed(3)}ms`);
  console.log(`WASM is ${result.improvement} faster`);

  t.pass();
});

test('Performance comparison - large input (1MB)', (t) => {
  const input = generateRandomString(1024 * 1024);
  const result = runBenchmark(input, 10);

  console.log('\nLarge input (1MB) performance:');
  console.log(`WASM MD5: ${result.wasmTime.toFixed(3)}ms`);
  console.log(`NPM MD5:  ${result.npmTime.toFixed(3)}ms`);
  console.log(`WASM is ${result.improvement} faster`);

  t.pass();
});

test('Performance comparison - very large input (10MB)', (t) => {
  const input = generateRandomString(10 * 1024 * 1024);
  const result = runBenchmark(input, 5);

  console.log('\nVery large input (10MB) performance:');
  console.log(`WASM MD5: ${result.wasmTime.toFixed(3)}ms`);
  console.log(`NPM MD5:  ${result.npmTime.toFixed(3)}ms`);
  console.log(`WASM is ${result.improvement} faster`);

  t.pass();
});
