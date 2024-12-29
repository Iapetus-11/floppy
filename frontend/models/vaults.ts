export type Vault = {
    id: string,
    name: string,
    provider: string,
    data: Record<string, string>,
};

export type VaultFile = {
    id: string,
    vault_id: string,
    path_id: string,
    name: string,
    file_type: 'file' | 'folder',
    parent_id: string | null,
    created_at: string | null,
    size: number | null,
};