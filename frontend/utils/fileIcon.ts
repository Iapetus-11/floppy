import { faFile, faFileAudio, faFileCode, faFileExcel, faFileImage, faFileLines, faFilePdf, faFilePowerpoint, faFileVideo, faFileWord, faFileZipper } from "@fortawesome/free-regular-svg-icons";

export function getFileIcon(fileName: string) {
    const fileParts = fileName.split('.');
    const fileExtension = fileParts.at(-1) || '';

    if (['png', 'apng', 'avif', 'gif', 'jpg', 'jpeg', 'jfif', 'pjpeg', 'pjp', 'svg', 'webp', 'bmp', 'tiff', 'tif'].includes(fileExtension)) {
        return faFileImage;
    }

    if (['webm', 'mkv', 'flv', 'vob', 'ogv', 'ogg', 'gifv', 'avi', 'mov', 'qt', 'mp4', 'm4p', 'm4v', 'mpg', 'mp2', 'mpeg', 'mpe', 'mpv'].includes(fileExtension)) {
        return faFileVideo;
    }

    if (['m4a', 'flac', 'mp3', 'wav', 'caf', 'aiff', 'aif', 'aifc'].includes(fileExtension)) {
        return faFileAudio;
    }

    if ('pdf' === fileExtension) {
        return faFilePdf;
    }

    if (['docx', 'doc', 'dot', 'dotx'].includes(fileExtension)) {
        return faFileWord;
    }

    if (['txt', 'md', 'log'].includes(fileExtension)) {
        return faFileLines;
    }

    if (['gz', 'tar', 'zip', '7z'].includes(fileExtension)) {
        return faFileZipper;
    }

    if (['py', 'c', 'nim', 'vue', 'ts', 'js', 'json', 'rs', 'cs', 'cpp', 'h', 'sql', 'html', 'css', 'mjs', 'sh', 'applescript', 'asm', 'bat', 'cmd', 'fs', 'go', 'php', 'swift'].includes(fileExtension)) {
        return faFileCode;
    }

    if (['xlsx', 'xlsb', 'xls', 'xlsm'].includes(fileExtension)) {
        return faFileExcel;
    }

    if (['pptx', 'ppt'].includes(fileExtension)) {
        return faFilePowerpoint;
    }

    return faFile;
}