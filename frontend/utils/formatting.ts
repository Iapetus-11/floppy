export function formatFileSize(bytes: number): string {
    let max = 1024;

    if (bytes < max) {
        return `${bytes.toFixed(0)} ${bytes === 1 ? 'byte' : 'bytes'}`
    }
    max *= 1024;

    if (bytes < max) {
        return `${(bytes / (max / 1024)).toFixed(0)} KB`;
    }
    max *= 1024;

    if (bytes < max) {
        return `${(bytes / (max / 1024)).toFixed(0)} MB`;
    }
    max *= 1024;

    return `${(bytes / (max / 1024)).toFixed(0)} GB`;
}

export function formatDuration(seconds: number): string {
    function p(n: number) {
        return n === 1 ? '' : 's'
    }

    if (seconds < (60 * 2)) {
        return `${seconds.toFixed(0)} second${p(seconds)}`;
    }

    const minutes = seconds / 60;
    if (minutes < (60 * 2)) {
        return `${minutes.toFixed(0)} minute${p(minutes)}`;
    }

    const hours = minutes / 60;
    if (hours < (24 * 2)) {
        return `${hours.toFixed(0)} hour${p(hours)}`;
    }

    const days = hours / 24;
    if (days < (7 * 2)) {
        return `${days.toFixed(1)} day${p(days)}`;
    }

    const weeks = days / 7;
    if (weeks < 52) {
        return `${weeks.toFixed(0)} week${p(weeks)}`;
    }

    const years = weeks / 52;

    return `${years.toFixed(1)} year${p(years)}`;
}