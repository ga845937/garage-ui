export interface BucketListItem {
  id: string;
  global_aliases: string[];
  local_aliases: LocalAlias[];
  objects: number;
  bytes: number;
  created: string;
}

export interface Bucket extends BucketListItem {
  website_access: boolean;
  website_config?: string;
  keys: BucketKey[];
  quotas: Quotas;
}

export interface LocalAlias {
  access_key_id: string;
  alias: string;
}

export interface BucketKey {
  access_key_id: string;
  name: string;
  permissions: BucketKeyPermissions;
  bucket_local_aliases: boolean;
}

export interface BucketKeyPermissions {
  read: boolean;
  write: boolean;
  owner: boolean;
}

export interface Quotas {
  max_size?: number;
  max_objects?: number;
}

export interface WebsiteConfig {
  index_document?: string;
  error_document?: string;
}

export interface LocalAliasInput {
  access_key_id: string;
  alias: string;
  allow: BucketKeyPermissions;
}

export interface RemoveLocalAliasInput {
  access_key_id: string;
  local_alias: string;
}
