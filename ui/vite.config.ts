import { defineConfig } from 'vite';
import checker from 'vite-plugin-checker';

const components = [
  'dialog',
  'dropdown',
  'menu',
  'menu-item',
  'checkbox',
  'divider',
  'menu-label',
  'option',
  'select',
  'tooltip',
  'card',
  'icon-button',
  'button',
  'icon',
  'alert',
  'input',
  'spinner',
  'avatar',
  'skeleton',
];
const exclude = components.map(
  c => `@shoelace-style/shoelace/dist/components/${c}/${c}.js`
);

// https://vitejs.dev/config/
export default defineConfig({
  optimizeDeps: {
    exclude: [
      ...exclude,
    ],
  },
  plugins: [
    checker({
      typescript: true,
      eslint: {
        lintCommand: 'eslint --ext .ts,.html . --ignore-path .gitignore',
      },
    }),
  ]
});
