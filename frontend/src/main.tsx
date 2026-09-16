import React from 'react';
import ReactDOM from 'react-dom/client';
import { QueryClientProvider } from '@tanstack/react-query';
import { HashRouter } from 'react-router-dom';
import { App } from './App';
import { queryClient } from './api/client';
import { AuthBoundary } from './auth/AuthBoundary';
import { ContentAccessBoundary } from './content-access/ContentAccessBoundary';
import './style.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <AuthBoundary>
      <ContentAccessBoundary>
        <QueryClientProvider client={queryClient}>
          <HashRouter><App /></HashRouter>
        </QueryClientProvider>
      </ContentAccessBoundary>
    </AuthBoundary>
  </React.StrictMode>,
);
