import type { PhiDocSiteConfig } from '@phiace/phidoc';

const config: PhiDocSiteConfig = {
  title: 'Phinet',
  description: 'A Fourier tensor engine for neural operators on the circle — complex autograd, a differentiable unitary FFT, Fourier neural operators, joint training, holographic binding and sixty-four spectral gates, with a PyTorch twin checked to 1e-16.',
  version: '0.1.0',
  brandId: 'phinet',
  docs: {
    path: '../docs',
    routeBasePath: 'docs',
    include: ['**/*.md', '**/*.mdx'],
    exclude: ['**/node_modules/**', '**/.git/**', '**/_*/**'],
  },
  home: {
    title: 'Phinet',
    tagline: 'In Phient’s world a word is a seed of what it relates to, under what conditions. Phinet computes those relations: functions on a closed circle, operators from functions to functions, every gradient checked — in Rust, and in PyTorch to the last digit.',
    actions: [
      { label: 'Quickstart', href: '/docs/get-started/quickstart', variant: 'primary' },
      { label: 'Every operation', href: '/docs/api/every-operation', variant: 'outline' },
      { label: 'In three dimensions', href: '/phinet/3d/index.html', variant: 'secondary', external: true },
    ],
    features: [
      { title: 'Get Started', description: 'Install, the sixty-second example line by line, and the cheat sheet from PyTorch or TensorFlow.', icon: 'rocket', href: '/docs/get-started/install' },
      { title: 'Tutorials', description: 'Seven, in order: a first operator, ranking a lexicon, the mixture objective, joint training, composition and cleanup, the sixty-four gates, spectral priors.', icon: 'book', href: '/docs/tutorials/first-operator' },
      { title: 'Guide', description: 'The tensor engine, the Fourier operator, objectives, autograd mechanics, numerical conventions, serialization, performance, holographic binding.', icon: 'layers', href: '/docs/guide/tensor-engine' },
      { title: 'API Reference', description: 'Every public item by module — Tensor, Tape, Adam, Fno, Joint, hrr, hexagram — with shapes, mathematics and the torch equivalent.', icon: 'code', href: '/docs/api/every-operation' },
      { title: 'PyTorch & Colab', description: 'The same operations in torch, 55 checks at 1e-16, a notebook that trains the operator, and the interactive 3-D page.', icon: 'sparkles', href: '/docs/interop/pytorch-parity' },
      { title: 'Measurements', description: 'Every held-out number, chance-normalised, beside the arm that differs in exactly one thing; and benchmarks on one machine.', icon: 'grid', href: '/docs/measurements/results' },
    ],
  },
  nav: [
    { label: 'Docs', href: '/docs/get-started/install' },
    { label: 'API', href: '/docs/api/every-operation' },
    { label: 'PyTorch', href: '/docs/interop/pytorch-parity' },
    { label: 'What is missing', href: '/docs/about/what-is-missing' },
    { label: 'GitHub', href: 'https://github.com/gemphi/phinet', external: true },
  ],
  sidebar: { mode: 'auto' },
  layout: { navbarSticky: true, navbarVariant: 'default', sidebarCollapsed: false },
};

export default config;
