/// <reference types="vite/client" />

declare module 'virtual:phidoc/content' {
  import type { PhiDocData } from '@phiace/phidoc';
  const data: PhiDocData;
  export default data;
}
