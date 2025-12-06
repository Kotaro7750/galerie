import type { AttributeMap } from '../types/filters'

export function normalizeTag(value: string): string {
  return value.trim().toLowerCase()
}

export function cloneAttributes(source: AttributeMap): AttributeMap {
  const entries = Object.entries(source).map(([key, values]) => [key, [...values]])
  return Object.fromEntries(entries)
}

export function cloneTags(source: string[]): string[] {
  return [...source]
}
