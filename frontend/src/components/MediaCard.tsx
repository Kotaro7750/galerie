import {
  Card,
  CardActions,
  CardContent,
  CardMedia,
  Chip,
  IconButton,
  Stack,
  Tooltip,
  Typography,
  Box,
} from '@mui/material'
import VisibilityRoundedIcon from '@mui/icons-material/VisibilityRounded'
import OpenInNewRoundedIcon from '@mui/icons-material/OpenInNewRounded'
import DownloadRoundedIcon from '@mui/icons-material/DownloadRounded'
import type { MouseEvent } from 'react'

import type { MediaSummary, MediaTag } from '../types/media'
import { resolveStreamUrl, resolveThumbnailUrl } from '../utils/mediaUrls'

type MediaCardProps = {
  media: MediaSummary
  apiBaseUrl: string
  onPreview: () => void
  onTagSelect: (tag: MediaTag) => void
}

export function MediaCard({ media, apiBaseUrl, onPreview, onTagSelect }: MediaCardProps) {
  const thumbnailSrc = resolveThumbnailUrl(media.thumbnailPath, apiBaseUrl)
  const inlineStreamUrl = resolveStreamUrl(apiBaseUrl, media.id)
  const downloadUrl = resolveStreamUrl(apiBaseUrl, media.id, 'attachment')

  const openNewTab = (event: MouseEvent) => {
    event.stopPropagation()
    if (typeof window === 'undefined') return
    window.open(inlineStreamUrl, '_blank', 'noopener,noreferrer')
  }

  return (
    <Card
      variant="outlined"
      sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}
      onClick={onPreview}
    >
      <CardMedia
        component="img"
        height={180}
        image={thumbnailSrc}
        alt={media.relativePath}
        sx={{ cursor: 'pointer' }}
        onError={(event) => {
          ;(event.target as HTMLImageElement).src = 'https://placehold.co/320x200?text=Media'
        }}
      />
      <CardContent sx={{ flexGrow: 1 }}>
        <Stack spacing={1}>
          <Stack direction="row" justifyContent="space-between" alignItems="center">
            <Box sx={{ minWidth: 0 }}>
              <Tooltip title={media.relativePath}>
                <Typography variant="subtitle2" noWrap sx={{ fontWeight: 600 }}>
                  {media.relativePath}
                </Typography>
              </Tooltip>
            </Box>
            <Chip size="small" label={media.mediaType.toUpperCase()} sx={{ textTransform: 'uppercase' }} />
          </Stack>
          <Typography variant="caption" color="text.secondary">
            {media.filesize.toLocaleString()} bytes
          </Typography>
          <Stack direction="row" spacing={1} flexWrap="wrap" useFlexGap>
            {media.tags.map((tag) => (
              <Chip
                key={tag.rawToken}
                label={tag.value ? `${tag.name}:${tag.value}` : tag.name}
                size="small"
                onClick={(event) => {
                  event.stopPropagation()
                  onTagSelect(tag)
                }}
                sx={{ mb: 0.5, cursor: 'pointer' }}
              />
            ))}
          </Stack>
        </Stack>
      </CardContent>
      <CardActions sx={{ justifyContent: 'flex-end', pt: 0 }}>
        <Tooltip title="Preview">
          <IconButton aria-label="Preview" onClick={(event) => { event.stopPropagation(); onPreview() }}>
            <VisibilityRoundedIcon fontSize="small" />
          </IconButton>
        </Tooltip>
        <Tooltip title="Open in new tab">
          <IconButton aria-label="Open in new tab" onClick={openNewTab}>
            <OpenInNewRoundedIcon fontSize="small" />
          </IconButton>
        </Tooltip>
        <Tooltip title="Download">
          <IconButton
            aria-label="Download"
            component="a"
            href={downloadUrl}
            target="_blank"
            rel="noopener noreferrer"
            onClick={(event) => event.stopPropagation()}
          >
            <DownloadRoundedIcon fontSize="small" />
          </IconButton>
        </Tooltip>
      </CardActions>
    </Card>
  )
}
