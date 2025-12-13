export interface AccessKeyBucketPermissions {
	read: boolean;
	write: boolean;
	owner: boolean;
}

export interface AccessKeyBucket {
	id: string;
	global_aliases: string[];
	local_aliases: string[];
	permissions: AccessKeyBucketPermissions;
}

export interface AccessKeyPermissions {
	read: boolean;
	write: boolean;
	owner: boolean;
}

export class AccessKeyListItem {
	public id!: string;
	public name!: string;
	public created?: string;
	public expiration?: string;
	public secret_access_key!: string;
}

export class AccessKey extends AccessKeyListItem {
	public permissions!: AccessKeyPermissions;
	public buckets!: AccessKeyBucket[];
}


