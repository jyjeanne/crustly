---
type: Rust Module
title: file
resource: src/services/file.rs#L1-L455
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/crate-db-models-file-repository-filerepository
  - external/crate-services-servicecontext
  - external/anyhow-context-result
  - external/chrono-utc
  - external/std-path-path-pathbuf
  - external/uuid-uuid
  - external/super
  - external/crate-services-sessionservice
  - external/crate-db-database
  member_of:
  - packages/crustly
---

# Contains

- [FileService](../../../classes/src/services/file/FileService.md)
- [new](../../../functions/src/services/file/FileService/new.md)
- [track_file](../../../functions/src/services/file/FileService/track_file.md)
- [get_file](../../../functions/src/services/file/FileService/get_file.md)
- [get_file_required](../../../functions/src/services/file/FileService/get_file_required.md)
- [list_files_for_session](../../../functions/src/services/file/FileService/list_files_for_session.md)
- [find_file_by_path](../../../functions/src/services/file/FileService/find_file_by_path.md)
- [update_file](../../../functions/src/services/file/FileService/update_file.md)
- [update_file_content](../../../functions/src/services/file/FileService/update_file_content.md)
- [delete_file](../../../functions/src/services/file/FileService/delete_file.md)
- [delete_files_for_session](../../../functions/src/services/file/FileService/delete_files_for_session.md)
- [count_files_in_session](../../../functions/src/services/file/FileService/count_files_in_session.md)
- [is_file_tracked](../../../functions/src/services/file/FileService/is_file_tracked.md)
- [get_or_create_file](../../../functions/src/services/file/FileService/get_or_create_file.md)
- [get_files_with_content](../../../functions/src/services/file/FileService/get_files_with_content.md)
- [get_files_without_content](../../../functions/src/services/file/FileService/get_files_without_content.md)
- [create_test_service](../../../functions/src/services/file/create_test_service.md)
- [test_track_file](../../../functions/src/services/file/test_track_file.md)
- [test_get_file](../../../functions/src/services/file/test_get_file.md)
- [test_list_files_for_session](../../../functions/src/services/file/test_list_files_for_session.md)
- [test_find_file_by_path](../../../functions/src/services/file/test_find_file_by_path.md)
- [test_update_file_content](../../../functions/src/services/file/test_update_file_content.md)
- [test_delete_file](../../../functions/src/services/file/test_delete_file.md)
- [test_delete_files_for_session](../../../functions/src/services/file/test_delete_files_for_session.md)
- [test_count_files_in_session](../../../functions/src/services/file/test_count_files_in_session.md)
- [test_is_file_tracked](../../../functions/src/services/file/test_is_file_tracked.md)
- [test_get_or_create_file](../../../functions/src/services/file/test_get_or_create_file.md)
- [test_get_files_with_content](../../../functions/src/services/file/test_get_files_with_content.md)

# Imports

- `crate::db::{models::File, repository::FileRepository}`
- `crate::services::ServiceContext`
- `anyhow::{Context, Result}`
- `chrono::Utc`
- `std::path::{Path, PathBuf}`
- `uuid::Uuid`
- `super::*`
- `crate::services::SessionService`
- `crate::db::Database`

# Member of

- [crustly](../../../packages/crustly.md)