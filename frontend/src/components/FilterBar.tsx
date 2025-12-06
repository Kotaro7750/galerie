import {
  Box,
  Chip,
  IconButton,
  InputAdornment,
  Stack,
  TextField,
  Tooltip,
} from '@mui/material'
import InfoOutlinedIcon from '@mui/icons-material/InfoOutlined'
import AddRoundedIcon from '@mui/icons-material/AddRounded'
import type { ChangeEvent, KeyboardEvent, RefObject } from 'react'

import type { AttributeMap } from '../types/filters'

type FilterBarProps = {
  tagDraft: string
  attrValue: string
  confirmedTags: string[]
  attributes: AttributeMap
  tagGuidance: string
  valueGuidance: string
  tagInputRef: RefObject<HTMLInputElement>
  onTagDraftChange: (value: string) => void
  onAttrValueChange: (value: string) => void
  onCommit: () => void
  onTagKeyDown: (event: KeyboardEvent<HTMLInputElement>) => void
  onRemoveTag: (tag: string) => void
  onRemoveAttribute: (key: string, value: string) => void
}

export function FilterBar({
  tagDraft,
  attrValue,
  confirmedTags,
  attributes,
  tagGuidance,
  valueGuidance,
  tagInputRef,
  onTagDraftChange,
  onAttrValueChange,
  onCommit,
  onTagKeyDown,
  onRemoveTag,
  onRemoveAttribute,
}: FilterBarProps) {
  const attributeChips = Object.entries(attributes)
  const hasAnyFilterChips = confirmedTags.length > 0 || attributeChips.length > 0

  return (
    <Stack spacing={3}>
      <Stack spacing={2}>
        <Stack direction={{ xs: 'column', md: 'row' }} spacing={1}>
          <TextField
            fullWidth
            label="Tag or key"
            placeholder="e.g. nature or camera"
            value={tagDraft}
            onChange={(event: ChangeEvent<HTMLInputElement>) => onTagDraftChange(event.target.value)}
            onKeyDown={onTagKeyDown}
            inputRef={tagInputRef}
            InputProps={{
              endAdornment: (
                <InputAdornment position="end">
                  <Tooltip title={tagGuidance}>
                    <IconButton
                      size="small"
                      edge="end"
                      aria-label="Tag entry tips"
                      tabIndex={-1}
                      disableRipple
                      disableFocusRipple
                    >
                      <InfoOutlinedIcon fontSize="small" />
                    </IconButton>
                  </Tooltip>
                </InputAdornment>
              ),
            }}
          />
          <TextField
            fullWidth
            label="Value (optional)"
            placeholder="e.g. nikon"
            value={attrValue}
            onChange={(event: ChangeEvent<HTMLInputElement>) => onAttrValueChange(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === 'Enter') {
                event.preventDefault()
                onCommit()
              }
            }}
            InputProps={{
              endAdornment: (
                <InputAdornment position="end">
                  <Tooltip title={valueGuidance}>
                    <IconButton
                      size="small"
                      edge="end"
                      aria-label="Value usage tips"
                      tabIndex={-1}
                      disableRipple
                      disableFocusRipple
                    >
                      <InfoOutlinedIcon fontSize="small" />
                    </IconButton>
                  </Tooltip>
                </InputAdornment>
              ),
            }}
          />
          <Box sx={{ display: 'flex', alignItems: 'center' }}>
            <Tooltip title={attrValue.trim() ? 'Add key:value filter' : 'Add tag'}>
              <span>
                <IconButton
                  color="primary"
                  onClick={onCommit}
                  disabled={!tagDraft.trim()}
                  sx={{ alignSelf: 'center' }}
                >
                  <AddRoundedIcon />
                </IconButton>
              </span>
            </Tooltip>
          </Box>
        </Stack>

        {hasAnyFilterChips && (
          <Stack direction="row" spacing={1} flexWrap="wrap" useFlexGap>
            {confirmedTags.map((tag) => (
              <Chip
                key={tag}
                label={tag}
                color="secondary"
                variant="outlined"
                onDelete={() => onRemoveTag(tag)}
                sx={{ mb: 1 }}
              />
            ))}
            {attributeChips.flatMap(([key, values]) =>
              values.map((value) => (
                <Chip
                  key={`${key}-${value}`}
                  label={`${key}:${value}`}
                  color="primary"
                  variant="outlined"
                  onDelete={() => onRemoveAttribute(key, value)}
                  sx={{ mb: 1 }}
                />
              )),
            )}
          </Stack>
        )}
      </Stack>
    </Stack>
  )
}
