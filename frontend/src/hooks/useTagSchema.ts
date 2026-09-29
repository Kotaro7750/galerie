import { useQuery } from '@tanstack/react-query';
import { apiRequest, tagSchemaApi } from '../api/client';

export function useTagSchema() {
  return useQuery({
    queryKey: ['tag-schema'],
    queryFn: ({ signal }) => apiRequest(tagSchemaApi.getTagSchema({ signal }), 'タグスキーマを取得できませんでした。'),
  });
}
