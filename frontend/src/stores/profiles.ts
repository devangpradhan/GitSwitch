import { writable, derived } from 'svelte/store';
import type { Profile } from '../types';
import { GetProfiles, SaveProfile as apiSaveProfile, DeleteProfile as apiDeleteProfile, SwitchActiveProfile } from '../wailsjs/go/main/App';
import { loadFolderMappings } from './mappings';
import { loadSSHKeys } from './ssh';

import { loadAppStatus } from './status';

export const profiles = writable<Profile[]>([]);
export const activeProfileId = writable<string>('');
export const isSwitching = writable<boolean>(false);
export const switchLatency = writable<number>(0);
export const switchMessage = writable<string>('');

export const activeProfile = derived(
  [profiles, activeProfileId],
  ([$profiles, $activeProfileId]) => {
    return $profiles.find(p => p.id === $activeProfileId) || $profiles[0] || null;
  }
);

export async function loadProfiles() {
  try {
    const list = await GetProfiles();
    profiles.set(list);
    return list;
  } catch (err) {
    console.error('Failed to load profiles:', err);
    return [];
  }
}

export async function switchProfile(profileId: string) {
  isSwitching.set(true);
  try {
    const res = await SwitchActiveProfile(profileId);
    activeProfileId.set(res.profile_id);
    switchLatency.set(res.latency_ms);
    switchMessage.set(res.message);
    await Promise.all([loadProfiles(), loadFolderMappings(), loadSSHKeys(), loadAppStatus()]);
    return res;
  } catch (err) {
    console.error('Failed to switch profile:', err);
    throw err;
  } finally {
    setTimeout(() => {
      isSwitching.set(false);
    }, 150);
  }
}

export async function saveProfile(profile: Profile) {
  try {
    await apiSaveProfile(profile);
    await Promise.all([loadProfiles(), loadFolderMappings(), loadSSHKeys(), loadAppStatus()]);
  } catch (err) {
    console.error('Failed to save profile:', err);
    throw err;
  }
}

export async function deleteProfile(profileId: string) {
  try {
    await apiDeleteProfile(profileId);
    await Promise.all([loadProfiles(), loadFolderMappings(), loadSSHKeys(), loadAppStatus()]);
  } catch (err) {
    console.error('Failed to delete profile:', err);
    throw err;
  }
}
