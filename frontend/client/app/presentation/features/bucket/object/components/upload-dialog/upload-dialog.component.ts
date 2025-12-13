import type { ElementRef } from "@angular/core";

import { CommonModule } from "@angular/common";
import { Component, inject, signal, ViewChild } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatDialogModule, MatDialogRef } from "@angular/material/dialog";
import { MatIconModule } from "@angular/material/icon";
import { NgxTolgeeModule } from "@tolgee/ngx";

export interface UploadDialogResult {
	files: FileWithPath[];
}

export interface FileWithPath {
	file: File;
	// biome-ignore lint/style/useNamingConvention: WebAPI naming
	relativePath: string;
}

@Component({
	imports: [
		CommonModule,
		MatButtonModule,
		MatDialogModule,
		MatIconModule,
		NgxTolgeeModule,
	],
	selector: "app-upload-dialog",
	standalone: true,
	// biome-ignore lint/style/useNamingConvention: Angular
	styleUrl: "./upload-dialog.component.scss",
	// biome-ignore lint/style/useNamingConvention: Angular
	templateUrl: "./upload-dialog.component.html",
})
export class UploadDialogComponent {
	private readonly dialog_ref = inject(MatDialogRef<UploadDialogComponent>);

	@ViewChild("fileInput") public file_input!: ElementRef<HTMLInputElement>;
	@ViewChild("folderInput")
	public folder_input!: ElementRef<HTMLInputElement>;

	public is_dragging = signal(false);

	public on_drag_over(event: DragEvent): void {
		event.preventDefault();
		event.stopPropagation();
		this.is_dragging.set(true);
	}

	public on_drag_leave(event: DragEvent): void {
		event.preventDefault();
		event.stopPropagation();
		this.is_dragging.set(false);
	}

	public async on_drop(event: DragEvent): Promise<void> {
		event.preventDefault();
		event.stopPropagation();
		this.is_dragging.set(false);

		const items = event.dataTransfer?.items;
		if (!items) {
			return;
		}

		const files: FileWithPath[] = [];

		const entries: FileSystemEntry[] = [];
		for (const item of Array.from(items)) {
			const entry = item.webkitGetAsEntry();
			if (entry) {
				entries.push(entry);
			}
		}

		// Recursively read all files from entries
		for (const entry of entries) {
			const entry_files = await this.read_entry(entry, "");
			files.push(...entry_files);
		}

		if (files.length > 0) {
			this.dialog_ref.close({ files } as UploadDialogResult);
		}
	}

	private async read_entry(
		entry: FileSystemEntry,
		base_path: string,
	): Promise<FileWithPath[]> {
		const files: FileWithPath[] = [];

		if (entry.isFile) {
			const file_entry = entry as FileSystemFileEntry;
			const file = await this.get_file_from_entry(file_entry);
			const relative_path = base_path
				? `${base_path}/${file.name}`
				: file.name;
			const file_relative_path = relative_path;
			// biome-ignore lint/style/useNamingConvention: WebAPI interface
			files.push({ file, relativePath: file_relative_path });
		} else if (entry.isDirectory) {
			const dir_entry = entry as FileSystemDirectoryEntry;
			const dir_reader = dir_entry.createReader();
			const entries = await this.read_directory_entries(dir_reader);
			const new_base_path = base_path
				? `${base_path}/${entry.name}`
				: entry.name;

			for (const child_entry of entries) {
				const child_files = await this.read_entry(
					child_entry,
					new_base_path,
				);
				files.push(...child_files);
			}
		}

		return files;
	}

	private get_file_from_entry(
		file_entry: FileSystemFileEntry,
	): Promise<File> {
		return new Promise((resolve, reject) => {
			file_entry.file(resolve, reject);
		});
	}

	private read_directory_entries(
		dir_reader: FileSystemDirectoryReader,
	): Promise<FileSystemEntry[]> {
		return new Promise((resolve, reject) => {
			dir_reader.readEntries(resolve, reject);
		});
	}

	public open_file_picker(): void {
		this.file_input.nativeElement.click();
	}

	public open_folder_picker(): void {
		this.folder_input.nativeElement.click();
	}

	public on_files_selected(event: Event): void {
		const input = event.target as HTMLInputElement;
		if (!input.files || input.files.length === 0) {
			return;
		}

		const selected_files: FileWithPath[] = Array.from(input.files).map((file) => {
			const relative_path = file.name;
			// biome-ignore lint/style/useNamingConvention: WebAPI interface
			return { file, relativePath: relative_path };
		});

		this.dialog_ref.close({ files: selected_files } as UploadDialogResult);
	}

	public on_folder_selected(event: Event): void {
		const input = event.target as HTMLInputElement;
		if (!input.files || input.files.length === 0) {
			return;
		}

		const folder_files: FileWithPath[] = Array.from(input.files).map((file) => {
			// webkitRelativePath includes the folder name and path
			// biome-ignore lint/style/useNamingConvention: WebAPI property
			const relative_path = (file as File & { webkitRelativePath?: string })
				.webkitRelativePath || file.name;
			// biome-ignore lint/style/useNamingConvention: WebAPI interface
			return { file, relativePath: relative_path };
		});

		this.dialog_ref.close({ files: folder_files } as UploadDialogResult);
	}

	public close(): void {
		this.dialog_ref.close();
	}
}
