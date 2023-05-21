/**
 * @license
 * Copyright 2018 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */

import {summary as summary} from 'rollup-plugin-summary';
import {copy} from '@web/rollup-plugin-copy';
import {terser} from 'rollup-plugin-terser';
import resolve from '@rollup/plugin-node-resolve';
import replace from '@rollup/plugin-replace';
import { rollupPluginHTML as html } from "@web/rollup-plugin-html";

export default {
  output: {
    dir: 'dist'
  },
  onwarn(warning) {
    if (warning.code !== 'THIS_IS_UNDEFINED') {
      console.error(`(!) ${warning.message}`);
    }
  },
  plugins: [
    html({
      input: 'index.html'
    }),
    replace({'Reflect.decorate': 'undefined'}),
    resolve(),
    terser({
      ecma: 2020,
      module: true,
      warnings: true,
      mangle: {
        properties: {
          regex: /^__/,
        },
      },
    }),
    summary(),
    copy({
      patterns: ['assets/**']
    })
  ],
};
