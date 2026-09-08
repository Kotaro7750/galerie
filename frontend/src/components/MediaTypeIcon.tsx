import { File, FileText, Image, Music, Video } from 'lucide-react';

export function MediaTypeIcon({ mediaType }: { mediaType: string }) {
  const type = mediaType.split(';', 1)[0].trim().toLowerCase();
  const kind = type.startsWith('image/') ? '画像' : type.startsWith('video/') ? '動画' : type.startsWith('audio/') ? '音声' : type === 'application/pdf' ? 'PDF' : 'ファイル';
  const Icon = kind === '画像' ? Image : kind === '動画' ? Video : kind === '音声' ? Music : kind === 'PDF' ? FileText : File;
  return <span className="tooltip" data-tip={mediaType}>
    <Icon role="img" aria-hidden={false} aria-label={`${kind} (${mediaType})`} className="size-5" strokeWidth={1.75} />
  </span>;
}
