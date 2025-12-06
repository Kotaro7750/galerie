import {
  Alert,
  Box,
  Card,
  CardContent,
  CircularProgress,
  Divider,
  Stack,
  Typography,
} from '@mui/material'
import type { MutableRefObject } from 'react'

import type { MediaSummary, MediaTag } from '../types/media'
import { MediaCard } from './MediaCard'

type SearchResultsProps = {
  totalResults: number
  items: MediaSummary[]
  apiBaseUrl: string
  loadMoreRef: MutableRefObject<HTMLDivElement | null>
  isInitialLoading: boolean
  isRefreshing: boolean
  isFetchingNextPage: boolean
  hasNextPage?: boolean
  onPreview: (index: number) => void
  onTagSelect: (tag: MediaTag) => void
}

export function SearchResults({
  totalResults,
  items,
  apiBaseUrl,
  loadMoreRef,
  isInitialLoading,
  isRefreshing,
  isFetchingNextPage,
  hasNextPage,
  onPreview,
  onTagSelect,
}: SearchResultsProps) {
  return (
    <Card variant="outlined">
      <CardContent>
        <Stack direction="row" justifyContent="space-between" alignItems="center" mb={2}>
          <Typography variant="h6" sx={{ fontWeight: 600 }}>
            Results ({totalResults})
          </Typography>
          {(isInitialLoading || isRefreshing) && (
            <Stack direction="row" spacing={1} alignItems="center" color="text.secondary">
              <CircularProgress size={18} thickness={5} />
              <Typography variant="body2">
                {isInitialLoading ? 'Loading media…' : 'Fetching latest media…'}
              </Typography>
            </Stack>
          )}
        </Stack>
        <Divider sx={{ mb: 2 }} />
        {isInitialLoading ? (
          <Stack alignItems="center" spacing={2} py={6}>
            <CircularProgress />
            <Typography variant="body2" color="text.secondary">
              Loading catalog…
            </Typography>
          </Stack>
        ) : items.length === 0 ? (
          <Alert severity="info">No media matched. Add filters or try browsing without tags.</Alert>
        ) : (
          <>
            <Box
              sx={{
                display: 'grid',
                gap: 2,
                gridTemplateColumns: {
                  xs: 'repeat(1, minmax(0, 1fr))',
                  sm: 'repeat(2, minmax(0, 1fr))',
                  md: 'repeat(3, minmax(0, 1fr))',
                  lg: 'repeat(4, minmax(0, 1fr))',
                },
              }}
            >
              {items.map((media, index) => (
                <Box key={media.id}>
                  <MediaCard
                    media={media}
                    apiBaseUrl={apiBaseUrl}
                    onPreview={() => onPreview(index)}
                    onTagSelect={onTagSelect}
                  />
                </Box>
              ))}
            </Box>
            <Box ref={loadMoreRef} sx={{ height: 8 }} />
            {isFetchingNextPage && (
              <Stack direction="row" spacing={1} alignItems="center" justifyContent="center" mt={3}>
                <CircularProgress size={18} thickness={5} />
                <Typography variant="body2" color="text.secondary">
                  Loading more…
                </Typography>
              </Stack>
            )}
            {!hasNextPage && items.length > 0 && (
              <Typography variant="caption" color="text.secondary" display="block" textAlign="center" mt={3}>
                End of results
              </Typography>
            )}
          </>
        )}
      </CardContent>
    </Card>
  )
}
