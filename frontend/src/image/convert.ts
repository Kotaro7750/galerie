export function convertToAvif(source: Blob, signal: AbortSignal): Promise<Blob> {
  return new Promise((resolve, reject) => {
    const worker = new Worker(new URL('./encode_avif.worker.ts', import.meta.url), { type: 'module' });
    const abort = () => { worker.terminate(); reject(new DOMException('Aborted', 'AbortError')); };
    signal.addEventListener('abort', abort, { once: true });
    if (signal.aborted) { abort(); return; }
    worker.onmessage = (event: MessageEvent<{ blob?: Blob; error?: string }>) => {
      signal.removeEventListener('abort', abort);
      worker.terminate();
      if (event.data.blob) resolve(event.data.blob);
      else reject(new Error(event.data.error || '画像を変換できませんでした。'));
    };
    worker.onerror = () => {
      signal.removeEventListener('abort', abort);
      worker.terminate();
      reject(new Error('画像を変換できませんでした。'));
    };
    worker.postMessage(source);
  });
}
