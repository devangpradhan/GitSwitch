import type { Profile, FolderMapping, SSHKeyInfo, SwitchResult, SSHProbeResult, HookValidationResult, ConfigFiles, AppStatus } from '../../../types';

export function GetProfiles(): Promise<Profile[]>;
export function SaveProfile(profile: Profile): Promise<boolean>;
export function DeleteProfile(profileId: string): Promise<boolean>;
export function SwitchActiveProfile(profileId: string): Promise<SwitchResult>;
export function GetSSHKeys(): Promise<SSHKeyInfo[]>;
export function GenerateSSHKey(keyName: string, email: string): Promise<SSHKeyInfo>;
export function TestSSHConnection(keyName: string): Promise<SSHProbeResult>;
export function GetFolderMappings(): Promise<FolderMapping[]>;
export function SaveFolderMapping(mapping: FolderMapping): Promise<boolean>;
export function DeleteFolderMapping(id: string): Promise<boolean>;
export function ValidateHook(path: string): Promise<HookValidationResult>;
export function GetStatus(): Promise<AppStatus>;
export function GetConfigFiles(): Promise<ConfigFiles>;
export function SyncKeychain(): Promise<string>;
export function FlushInactiveKeys(): Promise<number>;
export function ApplyToTerminal(profileId: string): Promise<string>;
export function SetDaemonMode(strict: boolean): Promise<string>;
