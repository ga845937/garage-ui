export interface S3Object {
  key: string;
  size: number;
  last_modified: string;
  etag: string;
  storage_class: string;
}

export interface ObjectList {
  data: S3Object[];
  next_continuation_token?: string;
  is_truncated: boolean;
  prefix?: string;
}