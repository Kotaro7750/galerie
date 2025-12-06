import { useEffect, useRef } from 'react'
import {
  Box,
  Card,
  Chip,
  IconButton,
  Portal,
  Stack,
  Tooltip,
  Typography,
} from '@mui/material'
import CloseRoundedIcon from '@mui/icons-material/CloseRounded'
import OpenInNewRoundedIcon from '@mui/icons-material/OpenInNewRounded'
import DownloadRoundedIcon from '@mui/icons-material/DownloadRounded'
import FullscreenRoundedIcon from '@mui/icons-material/FullscreenRounded'

import type { MediaSummary, MediaTag } from '../types/media'
import { resolveStreamUrl } from '../utils/mediaUrls'

type MediaPreviewOverlayProps = {
  media: MediaSummary | null
  apiBaseUrl: string
  onClose: () => void
  onNavigate: (direction: 'next' | 'previous') => void
  onTagSelect: (tag: MediaTag) => void
}

export function MediaPreviewOverlay({ media, apiBaseUrl, onClose, onNavigate, onTagSelect }: MediaPreviewOverlayProps) {
  const containerRef = useRef<HTMLDivElement | null>(null)
  const swipeStartX = useRef<number | null>(null)

  useEffect(() => {
    const handleKeyDown = (event: WindowEventMap['keydown']) => {
      if (event.key === 'Escape') {
        onClose()
        return
      }
      if (event.key === 'ArrowRight') {
        event.preventDefault()
        onNavigate('next')
        return
      }
      if (event.key === 'ArrowLeft') {
        event.preventDefault()
        onNavigate('previous')
      }
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [onClose, onNavigate])

  if (!media || typeof document === 'undefined') {
    return null
  }

  const inlineStreamUrl = resolveStreamUrl(apiBaseUrl, media.id)
  const downloadUrl = resolveStreamUrl(apiBaseUrl, media.id, 'attachment')

  const handleFullscreen = () => {
    const node = containerRef.current
    if (node?.requestFullscreen) {
      node.requestFullscreen().catch(() => {
        /* ignore */
      })
    }
  }

  const handleBackdropClick = (event: React.MouseEvent<HTMLDivElement>) => {
    if (event.target === event.currentTarget) {
      onClose()
    }
  }

  const handlePointerDown = (event: React.PointerEvent<HTMLDivElement>) => {
    swipeStartX.current = event.clientX
  }

  const handlePointerUp = (event: React.PointerEvent<HTMLDivElement>) => {
    if (swipeStartX.current === null) {
      return
    }
    const deltaX = event.clientX - swipeStartX.current
    const threshold = 40
    if (Math.abs(deltaX) > threshold) {
      if (deltaX > 0) {
        onNavigate('previous')
      } else {
        onNavigate('next')
      }
    }
    swipeStartX.current = null
  }

  const handlePointerLeave = () => {
    swipeStartX.current = null
  }

  return (
    <Portal>
      <Box
        onClick={handleBackdropClick}
        sx={{
          position: 'fixed',
          inset: 0,
          bgcolor: 'rgba(8, 15, 30, 0.85)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: (theme) => theme.zIndex.modal,
          transition: 'opacity 120ms ease',
        }}
        onPointerDown={handlePointerDown}
        onPointerUp={handlePointerUp}
        onPointerLeave={handlePointerLeave}
        onPointerCancel={handlePointerLeave}
      >
        <Card
          ref={containerRef}
          sx={{
            width: 'min(90vw, 960px)',
            maxHeight: '90vh',
            display: 'flex',
            flexDirection: 'column',
            gap: 2,
            p: 3,
          }}
        >
          <Stack direction="row" alignItems="center" justifyContent="space-between">
            <Stack spacing={0.5}>
              <Typography variant="subtitle1" fontWeight={600} noWrap>
                {media.relativePath}
              </Typography>
              <Typography variant="caption" color="text.secondary">
                {media.mediaType.toUpperCase()} · {media.filesize.toLocaleString()} bytes
              </Typography>
            </Stack>
            <IconButton aria-label="Close preview" onClick={onClose} size="small">
              <CloseRoundedIcon />
            </IconButton>
          </Stack>

          <Stack direction="row" spacing={1} flexWrap="wrap" useFlexGap>
            {media.tags.map((tag) => (
              <Chip
                key={`${media.id}-${tag.rawToken}`}
                label={tag.value ? `${tag.name}:${tag.value}` : tag.name}
                size="small"
                onClick={() => onTagSelect(tag)}
                sx={{ cursor: 'pointer' }}
              />
            ))}
          </Stack>

          <Box
            sx={{
              flexGrow: 1,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              minHeight: { xs: 260, md: 380 },
              bgcolor: 'grey.900',
              borderRadius: 2,
              overflow: 'hidden',
            }}
          >
            {renderPreviewMedia(media, inlineStreamUrl)}
          </Box>

          <Stack direction="row" spacing={1} justifyContent="flex-end">
            <Tooltip title="Fullscreen">
              <IconButton aria-label="Fullscreen" onClick={handleFullscreen}>
                <FullscreenRoundedIcon />
              </IconButton>
            </Tooltip>
            <Tooltip title="Open in new tab">
              <IconButton
                aria-label="Open in new tab"
                component="a"
                href={inlineStreamUrl}
                target="_blank"
                rel="noopener noreferrer"
              >
                <OpenInNewRoundedIcon />
              </IconButton>
            </Tooltip>
            <Tooltip title="Download">
              <IconButton
                aria-label="Download"
                component="a"
                href={downloadUrl}
                target="_blank"
                rel="noopener noreferrer"
              >
                <DownloadRoundedIcon />
              </IconButton>
            </Tooltip>
          </Stack>
        </Card>
      </Box>
    </Portal>
  )
}

function renderPreviewMedia(media: MediaSummary, streamUrl: string) {
  const commonStyles = {
    maxWidth: '100%',
    maxHeight: '80vh',
  }

  switch (media.mediaType) {
    case 'image':
    case 'gif':
      return (
        <Box
          component="img"
          src={streamUrl}
          alt={media.relativePath}
          sx={{ ...commonStyles, objectFit: 'contain' }}
        />
      )
    case 'video':
      return <Box component="video" src={streamUrl} controls autoPlay muted loop sx={commonStyles} />
    case 'audio':
      return (
        <Stack spacing={2} alignItems="center" width="100%">
          <Typography variant="body2" color="text.secondary">
            Audio preview
          </Typography>
          <audio src={streamUrl} controls autoPlay style={{ width: '100%' }} />
        </Stack>
      )
    case 'pdf':
      return (
        <Box
          component="iframe"
          src={streamUrl}
          sx={{ border: 0, width: '100%', height: '70vh', bgcolor: 'white' }}
        />
      )
    default:
      return (
        <Stack spacing={2} alignItems="center">
          <Typography color="text.secondary">
            Preview not available. Use Open or Download to view this media.
          </Typography>
        </Stack>
      )
  }
}
