import { useCallback, useEffect, useRef, useState, type KeyboardEvent as ReactKeyboardEvent } from 'react'
import { Alert, Button, Card, CardContent, Snackbar, Stack } from '@mui/material'
import { useInfiniteQuery } from '@tanstack/react-query'

import { FilterBar } from '../components/FilterBar'
import { MediaPreviewOverlay } from '../components/MediaPreviewOverlay'
import { SearchResults } from '../components/SearchResults'
import { usePersistedFilters, type PersistedFilters } from '../hooks/usePersistedFilters'
import type { MediaSearchRequest } from '../services/mediaClient'
import { fetchMedia } from '../services/mediaClient'
import type { MediaTag } from '../types/media'
import { cloneAttributes, cloneTags, normalizeTag } from '../utils/filterUtils'

type SearchPageProps = {
  apiBaseUrl: string
}

export function SearchPage({ apiBaseUrl }: SearchPageProps) {
  const { filters, setTags: persistTags, setAttributes: persistAttributes } = usePersistedFilters({
    tags: [],
    attributes: {},
  })
  const confirmedTags = filters.tags
  const [tagDraft, setTagDraft] = useState('')
  const [attrValue, setAttrValue] = useState('')
  const tagInputRef = useRef<HTMLInputElement | null>(null)
  const tagGuidance = 'Press Enter or comma to confirm plain tags. Provide a value to create key:value filters.'
  const valueGuidance = 'When a value is present, the left input is treated as the key (camera + nikon => camera:nikon).'
  const attributes = filters.attributes
  const [toastOpen, setToastOpen] = useState(false)
  const [previewIndex, setPreviewIndex] = useState<number | null>(null)
  const [appliedFilters, setAppliedFilters] = useState<PersistedFilters>(() => ({
    tags: cloneTags(filters.tags),
    attributes: cloneAttributes(filters.attributes),
  }))
  const loadMoreRef = useRef<HTMLDivElement | null>(null)
  const appliedTags = appliedFilters.tags
  const searchQuery = useInfiniteQuery({
    queryKey: ['media-search', apiBaseUrl, appliedFilters],
    initialPageParam: 1,
    queryFn: ({ pageParam }) => {
      const payload: MediaSearchRequest = {
        attributes: appliedFilters.attributes,
        page: pageParam,
        pageSize: 60,
      }
      if (appliedTags.length > 0) {
        payload.tags = appliedTags
      }
      return fetchMedia(payload, apiBaseUrl)
    },
    getNextPageParam: (lastPage) => {
      const hasMore = lastPage.page * lastPage.pageSize < lastPage.total
      return hasMore ? lastPage.page + 1 : undefined
    },
  })
  const { data, fetchNextPage, hasNextPage, isFetching, isFetchingNextPage, isError, error, refetch, status } =
    searchQuery
  const flattenedItems = data?.pages.flatMap((page) => page.items) ?? []
  const itemsCount = flattenedItems.length
  const totalResults = data?.pages[0]?.total ?? 0
  const isInitialLoading = status === 'pending' && !data
  const isRefreshing = isFetching && !isFetchingNextPage
  const previewMedia = typeof previewIndex === 'number' ? flattenedItems[previewIndex] ?? null : null

  useEffect(() => {
    if (previewIndex === null) return
    if (itemsCount === 0) {
      setPreviewIndex(null)
      return
    }
    if (!flattenedItems[previewIndex]) {
      setPreviewIndex((prev) => {
        if (prev === null) return prev
        const nextIndex = Math.min(prev, itemsCount - 1)
        return nextIndex >= 0 ? nextIndex : null
      })
    }
  }, [flattenedItems, itemsCount, previewIndex])

  useEffect(() => {
    setToastOpen(false)
    setAppliedFilters({
      tags: cloneTags(filters.tags),
      attributes: cloneAttributes(filters.attributes),
    })
  }, [filters.attributes, filters.tags])

  const handleRemoveAttribute = (key: string, value: string) => {
    const values = attributes[key]?.filter((item) => item !== value) ?? []
    if (values.length === 0) {
      const nextAttributes = { ...attributes }
      delete nextAttributes[key]
      persistAttributes(nextAttributes)
    } else {
      persistAttributes({ ...attributes, [key]: values })
    }
  }

  const openPreviewAt = useCallback((index: number) => {
    setPreviewIndex(index)
  }, [])

  const closePreview = useCallback(() => setPreviewIndex(null), [])

  const handleNavigatePreview = useCallback(
    (direction: 'next' | 'previous') => {
      setPreviewIndex((current) => {
        if (current === null || itemsCount === 0) {
          return current
        }
        if (direction === 'next') {
          return (current + 1) % itemsCount
        }
        return (current - 1 + itemsCount) % itemsCount
      })
    },
    [itemsCount],
  )

  const handleCommitTagOrAttribute = useCallback(() => {
    const normalized = normalizeTag(tagDraft)
    const value = attrValue.trim()
    if (!normalized) {
      setTagDraft('')
      return
    }
    const focusTagInput = () => tagInputRef.current?.focus()
    if (value) {
      const existing = attributes[normalized] ?? []
      if (existing.includes(value)) {
        setAttrValue('')
        setTagDraft('')
        focusTagInput()
        return
      }
      persistAttributes({ ...attributes, [normalized]: [...existing, value] })
      setAttrValue('')
      setTagDraft('')
      focusTagInput()
      return
    }
    if (confirmedTags.includes(normalized)) {
      setTagDraft('')
      setAttrValue('')
      focusTagInput()
      return
    }
    persistTags([...confirmedTags, normalized])
    setTagDraft('')
    setAttrValue('')
    focusTagInput()
  }, [attrValue, attributes, confirmedTags, persistAttributes, persistTags, tagDraft])

  const handleRemoveTag = useCallback(
    (tag: string) => {
      persistTags(confirmedTags.filter((item) => item !== tag))
    },
    [confirmedTags, persistTags],
  )

  const handleAppendTagFromCard = useCallback(
    (tag: MediaTag) => {
      if (tag.type === 'keyvalue' && tag.value) {
        const key = normalizeTag(tag.name)
        const value = tag.value.trim()
        if (!key || !value) {
          return
        }
        const existingValues = attributes[key] ?? []
        if (existingValues.includes(value)) {
          return
        }
        persistAttributes({ ...attributes, [key]: [...existingValues, value] })
        tagInputRef.current?.focus()
        return
      }
      const normalized = normalizeTag(tag.normalized || tag.name)
      if (!normalized || confirmedTags.includes(normalized)) {
        return
      }
      persistTags([...confirmedTags, normalized])
      tagInputRef.current?.focus()
    },
    [attributes, confirmedTags, persistAttributes, persistTags],
  )

  const handleTagKeyDown = (event: ReactKeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter' || event.key === ',') {
      event.preventDefault()
      handleCommitTagOrAttribute()
    }
  }

  useEffect(() => {
    if (!isError) return
    const id = window.setTimeout(() => setToastOpen(true), 0)
    return () => window.clearTimeout(id)
  }, [isError])

  useEffect(() => {
    const node = loadMoreRef.current
    if (!node || !hasNextPage) {
      return
    }
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && hasNextPage && !isFetchingNextPage) {
          fetchNextPage()
        }
      },
      { rootMargin: '200px 0px' },
    )
    observer.observe(node)
    return () => observer.disconnect()
  }, [fetchNextPage, hasNextPage, isFetchingNextPage])

  return (
    <Stack spacing={4} sx={{ py: { xs: 4, md: 6 } }}>
      <Card variant="outlined">
        <CardContent>
          <FilterBar
            tagDraft={tagDraft}
            attrValue={attrValue}
            confirmedTags={confirmedTags}
            attributes={attributes}
            tagGuidance={tagGuidance}
            valueGuidance={valueGuidance}
            tagInputRef={tagInputRef}
            onTagDraftChange={setTagDraft}
            onAttrValueChange={setAttrValue}
            onCommit={handleCommitTagOrAttribute}
            onTagKeyDown={handleTagKeyDown}
            onRemoveTag={handleRemoveTag}
            onRemoveAttribute={handleRemoveAttribute}
          />
        </CardContent>
      </Card>

      <SearchResults
        totalResults={totalResults}
        items={flattenedItems}
        apiBaseUrl={apiBaseUrl}
        loadMoreRef={loadMoreRef}
        isInitialLoading={isInitialLoading}
        isRefreshing={isRefreshing}
        isFetchingNextPage={isFetchingNextPage}
        hasNextPage={hasNextPage}
        onPreview={openPreviewAt}
        onTagSelect={handleAppendTagFromCard}
      />

      <Snackbar
        open={toastOpen}
        autoHideDuration={5000}
        onClose={() => setToastOpen(false)}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
      >
        <Alert
          severity="error"
          onClose={() => setToastOpen(false)}
          action={
            <Button
              color="inherit"
              size="small"
              onClick={() => {
                setToastOpen(false)
                refetch()
              }}
            >
              Retry
            </Button>
          }
        >
          {resolveErrorMessage(error)}
        </Alert>
      </Snackbar>
      {previewMedia && (
        <MediaPreviewOverlay
          media={previewMedia}
          apiBaseUrl={apiBaseUrl}
          onClose={closePreview}
          onNavigate={handleNavigatePreview}
          onTagSelect={handleAppendTagFromCard}
        />
      )}
    </Stack>
  )
}

function resolveErrorMessage(error: unknown) {
  if (error instanceof Error) {
    return error.message
  }
  return 'Unable to load media results'
}
