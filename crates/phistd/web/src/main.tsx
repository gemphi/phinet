import React from 'react';
import ReactDOM from 'react-dom/client';
import { PhiDocSite } from '@phiace/phidoc';
import '@phiace/puijs/styles';
import data from 'virtual:phidoc/content';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <PhiDocSite data={data} />
  </React.StrictMode>
);
