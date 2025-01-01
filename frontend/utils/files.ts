export async function downloadFile(url: string, name: string, options: { headers: Record<string, string> } & any) {
    const fileBlob: Blob = await $fetch(url, { ...options, responseType: 'blob' });

    const fileBlobUrl = URL.createObjectURL(fileBlob);
    
    const aEl = document.createElement('a');
    aEl.download = name;
    aEl.href = fileBlobUrl;

    aEl.click();

    URL.revokeObjectURL(fileBlobUrl);
    aEl.remove();
}