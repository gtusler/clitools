use clap::{builder::PathBufValueParser, Arg, ArgAction, Command, ValueHint};
use std::path::PathBuf;

fn main() {
    let command = Command::new("filers")
        .about("Infer a file type based on magic number signatures.")
        .arg(
            Arg::new("path")
                .help("The file path")
                .required(true)
                .value_parser(PathBufValueParser::new())
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("supported")
                .help("List supported file types")
                .long("list-supported")
                .exclusive(true)
                .action(ArgAction::SetTrue),
        );

    let matches = command.get_matches();

    let list_supported = matches.get_one::<bool>("supported").expect("has default");

    if *list_supported {
        print_supported_list();
        return;
    }

    let file_path = matches.get_one::<PathBuf>("path").expect("required arg");

    let kind = infer::get_from_path(file_path)
        .expect("file read successfully")
        .expect("file type is known");

    println!("mime: {}", kind.mime_type());
    println!("extension: {}", kind.extension());
}

fn print_supported_list() {
    println!(
        r#"
Images
    - jpg    image/jpg
    - png    image/png
    - gif    image/gif
    - webp   image/webp
    - cr2    image/x-canon-cr2
    - tif    image/tiff
    - bmp    image/bmp
    - heif   image/heif
    - avif   image/avif
    - jxr    image/vnd.ms-photo
    - psd    image/vnd.adobe.photoshop
    - ico    image/vnd.microsoft.icon
    - ora    image/openraster
    - djvu   image/vnd.djvu

Video
    - mp4    video/mp4
    - m4v    video/x-m4v
    - mkv    video/x-matroska
    - webm   video/webm
    - mov    video/quicktime
    - avi    video/x-msvideo
    - wmv    video/x-ms-wmv
    - mpg    video/mpeg
    - flv    video/x-flv

Audio
    - mid    audio/midi
    - mp3    audio/mpeg
    - m4a    audio/mp4a
    - ogg    audio/ogg
    - flac   audio/x-flac
    - wav    audio/x-wav
    - amr    audio/amr
    - aac    audio/aac
    - aiff   audio/x-aiff
    - dsf    audio/x-dsf
    - ape    audio/x-ape

Archive
    - epub   application/epub+zip
    - zip    application/zip
    - tar    application/x-tar
    - rar    application/vnd.rar
    - gz     application/gzip
    - bz2    application/x-bzip2
    - 7z     application/x-7z-compressed
    - xz     application/x-xz
    - pdf    application/pdf
    - swf    application/x-shockwave-flash
    - rtf    application/rtf
    - eot    application/octet-stream
    - ps     application/postscript
    - sqlite application/vnd.sqlite3
    - nes    application/x-nintendo-nes-rom
    - crx    application/x-google-chrome-extension
    - cab    application/vnd.ms-cab-compressed
    - deb    application/vnd.debian.binary-package
    - ar     application/x-unix-archive
    - Z      application/x-compress
    - lz     application/x-lzip
    - rpm    application/x-rpm
    - dcm    application/dicom
    - zst    application/zstd
    - msi    application/x-ole-storage
    - cpio   application/x-cpio

Book
    - epub   application/epub+zip
    - mobi   application/x-mobipocket-ebook

Documents
    - doc    application/msword
    - docx   application/vnd.openxml-formats-officedocument.wordprocessingml.document
    - xls    application/vnd.ms-excel
    - xlsx   application/vnd.openxml-formats-officedocument.spreadsheetml.sheet
    - ppt    application/vnd.ms-powerpoint
    - pptx   application/vnd.openxml-formats-officedocument.presentationml.presentation
    - odt    application/vnd.oasis.opendocument.text
    - ods    application/vnd.oasis.opendocument.spreadsheet
    - odp    application/vnd.oasis.opendocument.presentation

Font
    - woff   application/font-woff
    - woff2  application/font-woff
    - ttf    application/font-sfnt
    - otf    application/font-sfnt

Application
    - wasm   application/wasm
    - exe    application/vnd.microsoft.portable-executable
    - dll    application/vnd.microsoft.portable-executable
    - elf    application/x-executable
    - bc     application/llvm
    - mach   application/x-mach-binary
    - class  application/java
    - dex    application/vnd.android.dex
    - dey    application/vnd.android.dey
    - der    application/x-x509-ca-cert
    - obj    application/x-executable
"#
    );
}
