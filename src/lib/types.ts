export type ContentType = 'text' | 'code' | 'url' | 'image' | 'color';

export interface Clip {
  id: string;
  content_type: ContentType;
  text_content?: string | null;
  title?: string | null;
  metadata?: string | null;
  category: string;
  color_hex?: string | null;
  thumbnail_data?: string | null;
  image_width?: number | null;
  image_height?: number | null;
  image_size_bytes?: number | null;
  is_pinned: boolean;
  created_at: number;
  last_used_at: number;
}

import type { Language } from './i18n';

export interface AppSettings {
  theme: 'pantheon-light' | 'pantheon-dark' | 'modern-flat-light' | 'modern-flat-dark';
  language: Language;
  clear_on_exit?: boolean;
  global_shortcut?: string;
  max_history?: number;
}

export type CategoryId = 'all' | 'text' | 'code' | 'url' | 'image' | 'favorites';
