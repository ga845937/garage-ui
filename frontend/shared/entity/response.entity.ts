export interface ListResponse<T> {
	rows: T[];
	total: number;
}