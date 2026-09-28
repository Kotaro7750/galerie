self.onmessage = async (event: MessageEvent<Blob>) => {
  try {
    const bitmap = await createImageBitmap(event.data);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const context = canvas.getContext('2d');
    if (!context) throw new Error('画像を変換できませんでした。');
    context.drawImage(bitmap, 0, 0);
    bitmap.close();
    try {
      const native = await canvas.convertToBlob({ type: 'image/avif', quality: 0.8 });
      if (native.type === 'image/avif') { self.postMessage({ blob: native }); return; }
    } catch { /* Browser does not provide AVIF encoding. */ }
    const { default: encode } = await import('@jsquash/avif/encode');
    const encoded = await encode(context.getImageData(0, 0, canvas.width, canvas.height), { quality: 80 });
    self.postMessage({ blob: new Blob([encoded], { type: 'image/avif' }) });
  } catch {
    self.postMessage({ error: 'この画像を AVIF に変換できませんでした。ブラウザで表示できる画像を選択してください。' });
  }
};
