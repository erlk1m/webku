/**
 * Translation dictionaries barrel export
 *
 * Each locale maps to its UI string dictionary.
 * The default locale (id) provides the complete set of keys;
 * other locales can be partial and will fall back to id.
 */

import type { DefaultUIStrings, UIStrings } from '../types';
import { uiStrings as en } from './en';
import { uiStrings as id } from './id';
import { uiStrings as ja } from './ja';
import { uiStrings as zh } from './zh';

/** All translation dictionaries indexed by locale code */
export const translations: Record<string, DefaultUIStrings | UIStrings> = {
  id,
  zh,
  en,
  ja,
};
