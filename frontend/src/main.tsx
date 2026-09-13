import React from 'react';
import ReactDOM from 'react-dom/client';
import { QueryClientProvider } from '@tanstack/react-query';
import { HashRouter } from 'react-router-dom';
import { App } from './App';
import { queryClient } from './api/client';
import { AuthBoundary } from './auth/AuthBoundary';
import './style.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <AuthBoundary>
      <QueryClientProvider client={queryClient}>
        <HashRouter><App /></HashRouter>
      </QueryClientProvider>
    </AuthBoundary>
  </React.StrictMode>,
);
