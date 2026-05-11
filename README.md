# AdaWorldAPI/Sharepoint-rs

SharePoint vertical for the spear hub.

This repository contains the Rust implementation and transcodes from the original C# Sharepoint project.

## Structure
- `crates/smb-qos/` - Quality of Service pipelines (ST-06, ST-17)
- `crates/smb-transport-sharepoint/` - SharePoint transport
- `crates/smb-graph-delta/` - Delta sync logic
- `crates/smb-policy-encryption/` - Encryption policies
- `crates/smb-policy-tenant-rbac/` - RBAC policies