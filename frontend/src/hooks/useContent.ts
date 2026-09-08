import { useQuery } from '@tanstack/react-query';
import { apiRequest, contentsApi } from '../api/client';

export function useContent(contentId: string) {
  return useQuery({
    queryKey: ['content', contentId],
    queryFn: ({ signal }) => apiRequest(contentsApi.getContent({ contentId }, { signal })),
  });
}
