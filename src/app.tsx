import './index.css'; // import css

import * as React from 'react';
import { createRoot } from 'react-dom/client';
import { HashRouter, Route, Routes } from 'react-router-dom';
import CertList from './ui/pages/cert-list';

const root = createRoot(document.getElementById('main') as HTMLElement);

root.render(
  <React.StrictMode>
    <HashRouter>
      <Routes>
        <Route path="/" element={<CertList />} />
      </Routes>
    </HashRouter>
  </React.StrictMode>
);
