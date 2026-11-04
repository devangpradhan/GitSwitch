import { writable } from 'svelte/store';
import type { FolderMapping, HookValidationResult } from '../types';
import { GetFolderMappings, SaveFolderMapping as apiSaveMapping, DeleteFolderMapping as apiDeleteMapping, ValidateHook } from '../wailsjs/go/main/App';

export const folderMappings = writable<FolderMapping[]>([]);
export const currentValidation = writable<HookValidationResult | null>(null);
export const isValidating = writable<boolean>(false);

export async function loadFolderMappings() {
  try {
    const list = await GetFolderMappings();
    folderMappings.set(list);
    return list;
  } catch (err) {
    console.error('Failed to load folder mappings:', err);
    return [];
  }
}

export async function saveFolderMapping(mapping: FolderMapping) {
  try {
    await apiSaveMapping(mapping);
    await loadFolderMappings();
  } catch (err) {
    console.error('Failed to save mapping:', err);
    throw err;
  }
}

export async function deleteFolderMapping(id: string) {
  try {
    await apiDeleteMapping(id);
    await loadFolderMappings();
  } catch (err) {
    console.error('Failed to delete mapping:', err);
    throw err;
  }
}

export async function validatePath(path: string) {
  isValidating.set(true);
  try {
    const result = await ValidateHook(path);
    currentValidation.set(result);
    return result;
  } catch (err) {
    console.error('Validation error:', err);
    throw err;
  } finally {
    isValidating.set(false);
  }
}
